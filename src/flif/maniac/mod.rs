use numbers::rac::ChanceTable;
use numbers::rac::Rac;
use std::io::Read;
use numbers::near_zero::NearZeroCoder;
use error::*;

pub struct ManiacTree {
    root: ManiacNode
}

impl ManiacTree {
    pub fn new<R: Read>(rac: &mut Rac<R>) -> ManiacTree {
        unimplemented!()
    }
}

enum ManiacNode {
    
    /// Denotes a property node, property nodes are nodes that currently act as leaf nodes but will become inner nodes when their counter reaches zero
    Property{id: usize, value: i32, table: ChanceTable, counter: u32, left: Box<InactiveManiacNode>, right: Box<InactiveManiacNode>},
    /// Inner nodes are property nodes whose counters have reached zero. They no longer have a context associated with them.
    Inner{id: usize, value: i32, left: Box<ManiacNode>, right: Box<ManiacNode>},
    /// Leaf nodes are nodes that can never become inner nodes
    Leaf(ChanceTable)
}

enum InactiveManiacNode {
    InactiveProperty{id: usize, value: i32, counter: u32, left: Box<InactiveManiacNode>, right: Box<InactiveManiacNode>},
    InactiveLeaf
}

impl ManiacNode {
    // return type is temporary, will be some reasonable pixel value
    pub fn apply<R: Read>(self, rac: &mut Rac<R>, pvec: Vec<i32>, min: i32, max: i32) -> Result<(Self, i32)> {
        use self::ManiacNode::*;
        match self {
            Property{id, value, left, right, mut counter, mut table} => {
                let val = rac.read_near_zero(min, max, &mut table)?;
                counter -= 1;
                if counter == 0 {
                    let left = Box::new(left.activate(table.clone()));
                    let right = Box::new(right.activate(table));
                    Ok((Inner{id, value, left, right}, val))
                } else {
                    Ok((Property{id, value, counter, left, right, table}, val))
                }
            },
            Inner{id, value, left, right} => {
                if pvec[id] > value {
                    left.apply(rac, pvec, min, max)
                } else {
                    right.apply(rac, pvec, min, max)
                }
            },
            Leaf(mut table) => {
                let val = rac.read_near_zero(min, max, &mut table)?;
                Ok((Leaf(table), val))
            },
        }
    }
}

impl InactiveManiacNode {
    pub fn activate(self, table: ChanceTable) -> ManiacNode {
        use self::InactiveManiacNode::*;
        use self::ManiacNode::*;
        match self {
            InactiveLeaf => {
                Leaf(table)
            },
            InactiveProperty{id, value, counter, left, right} => {
                Property{id, value, counter, table, left, right}
            }
        }
    }
}
