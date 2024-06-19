use nom::{error::ParseError, multi::length_count, number::complete::{be_u16, be_u32, be_u8}, IResult};

use crate::{U1, U2, U4};

#[derive(Debug)]
pub struct AttributeInfo {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub info: Vec<U1>,
}

pub fn attribute_info_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], AttributeInfo> {
	let (input, attribute_name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'attribute_name_index'");
	
	let (input, info) = length_count(be_u32::<&[u8], E>, be_u8::<&[u8], E>)(input).expect("Failed to read 'attribute_length' or 'info'");
	let attribute_length = info.len() as U4;

	Ok((input, AttributeInfo {
		attribute_name_index,
		attribute_length,
		info
	}))
}