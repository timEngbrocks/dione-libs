use nom::{error::ParseError, number::complete::be_u32, IResult};

use crate::{U1, U4};

use super::DOUBLE;

#[derive(Debug)]
pub struct Double {
    pub tag: U1,
    pub high_bytes: U4,
    pub low_bytes: U4,
}

pub fn double_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], Double> {
    let (input, high_bytes) = be_u32::<&[u8], E>(input).expect("Failed to read 'high_bytes'");
    let (input, low_bytes) = be_u32::<&[u8], E>(input).expect("Failed to read 'low_bytes'");

    Ok((input, Double {
        tag: DOUBLE,
        high_bytes,
        low_bytes,
    }))
}
