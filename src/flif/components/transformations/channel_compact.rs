use super::Transformation;
use numbers::rac::Rac;
use std::io::Read;
use numbers::near_zero::NearZeroCoder;
use components::header::{Header, SecondHeader};
use numbers::rac::{ChanceTable, ChanceTableEntry};
use error::*;

#[derive(Debug)]
pub struct ChannelCompact {
    max: [u16; 4],
    decompacted: [Vec<u16>; 4],
}
impl ChannelCompact {
    pub fn new<R: Read, T: ?Sized + Transformation>(
        rac: &mut Rac<R>,
        transformation: &T,
        (ref header, ref second): (&Header, &SecondHeader),
    ) -> Result<ChannelCompact> {
        let mut context = ChanceTable::new(second.alpha_divisor, second.cutoff);
        let mut t = ChannelCompact {
            max: [0; 4],
            decompacted: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        };

        for c in 0..header.channels as usize {
            t.max[c] = rac.read_near_zero(
                0,
                transformation.max(c as u8) - transformation.min(c as u8),
                &mut context,
            )?;
            let mut min = transformation.min(c as u8);
            for i in 0..t.max[c] {
                t.decompacted[c].push(
                    min
                        + rac.read_near_zero(
                            0,
                            transformation.max(c as u8) - (min + (t.max[c] - i)),
                            &mut context,
                        )?,
                );
                min = t.decompacted[c][i as usize];
            }
        }

        Ok(t)
    }
}

impl Transformation for ChannelCompact {
    fn snap(&self, channel: u8, values: u16, pixel: u16) -> u16 {
        unimplemented!()
    }

    fn min(&self, channel: u8) -> u16 {
        0
    }

    fn max(&self, channel: u8) -> u16 {
        self.max[channel as usize]
    }

    fn cmin(&self, channel: u8, values: u16) -> u16 {
        unimplemented!()
    }

    fn cmax(&self, channel: u8, values: u16) -> u16 {
        unimplemented!()
    }
}
