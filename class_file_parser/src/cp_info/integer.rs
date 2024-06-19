use nom::{error::ParseError, number::complete::be_u32, IResult};

use crate::{U1, U4};

use super::INTEGER;

#[derive(Debug)]
pub struct Integer {
    pub tag: U1,
    pub bytes: U4,
}

pub fn integer_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], Integer> {
    let (input, bytes) = be_u32::<&[u8], E>(input).expect("Failed to read 'bytes'");

    Ok((input, Integer {
        tag: INTEGER,
        bytes,
    }))
}
