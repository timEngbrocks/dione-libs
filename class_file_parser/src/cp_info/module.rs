use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::MODULE;

#[derive(Debug)]
pub struct Module {
    pub tag: U1,
    pub name_index: U2,
}

pub fn module_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], Module> {
    let (input, name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_index'");

    Ok((input, Module {
        tag: MODULE,
        name_index,
    }))
}
