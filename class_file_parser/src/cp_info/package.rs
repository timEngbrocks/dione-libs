use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::PACKAGE;

#[derive(Debug)]
pub struct Package {
    pub tag: U1,
    pub name_index: U2,
}

pub fn package_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], Package> {
    let (input, name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_index'");

    Ok((input, Package {
        tag: PACKAGE,
        name_index,
    }))
}
