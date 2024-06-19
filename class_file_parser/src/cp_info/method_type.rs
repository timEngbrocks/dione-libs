use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::METHODTYPE;

#[derive(Debug)]
pub struct MethodType {
    pub tag: U1,
    pub descriptor_index: U2,
}

pub fn methodtype_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], MethodType> {
    let (input, descriptor_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'descriptor_index'");

    Ok((input, MethodType {
        tag: METHODTYPE,
        descriptor_index,
    }))
}
