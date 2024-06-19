use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::NAMEANDTYPE;

#[derive(Debug)]
pub struct NameAndType {
    pub tag: U1,
    pub name_index: U2,
    pub descriptor_index: U2,
}

pub fn nameandtype_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], NameAndType> {
    let (input, name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_index'");
    let (input, descriptor_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'descriptor_index'");

    Ok((input, NameAndType {
        tag: NAMEANDTYPE,
        name_index,
        descriptor_index,
    }))
}
