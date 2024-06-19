use nom::{error::ParseError, multi::length_count, number::complete::{be_u16, be_u8}, IResult};

use crate::{U1, U2};

use super::UTF8;

#[derive(Debug)]
pub struct Utf8 {
    pub tag: U1,
    pub length: U2,
    pub bytes: Vec<U1>,
}

pub fn utf8_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], Utf8> {
    let (input, bytes) = length_count(be_u16::<&[u8], E>, be_u8::<&[u8], E>)(input).expect("Failed to read 'length' or 'bytes'");
	let length = bytes.len() as U2;

    Ok((input, Utf8 {
        tag: UTF8,
        length,
        bytes,
    }))
}
