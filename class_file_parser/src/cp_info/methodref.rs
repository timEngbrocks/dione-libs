use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::METHODREF;

#[derive(Debug)]
pub struct Methodref {
    pub tag: U1,
    pub class_index: U2,
    pub name_and_type_index: U2,
}

pub fn methodref_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], Methodref> {
    let (input, class_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'class_index'");
    let (input, name_and_type_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_and_type_index'");

    Ok((input, Methodref {
        tag: METHODREF,
        class_index,
        name_and_type_index,
    }))
}
