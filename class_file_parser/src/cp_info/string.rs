use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::STRING;

#[derive(Debug)]
pub struct String {
    pub tag: U1,
    pub string_index: U2,
}

pub fn string_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], String> {
    let (input, string_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'string_index'");

    Ok((input, String {
        tag: STRING,
        string_index,
    }))
}
