use nom::{error::ParseError, multi::length_count, number::complete::be_u16, IResult};

use crate::{attribute_info::{attribute_info_parser, AttributeInfo}, U2};

#[derive(Debug)]
pub struct FieldInfo {
	pub access_flags: U2,
	pub name_index: U2,
	pub descriptor_index: U2,
	pub attributes_count: U2,
	pub attributes: Vec<AttributeInfo>,
}

pub fn field_info_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], FieldInfo> {
	let (input, access_flags) = be_u16::<&[u8], E>(input).expect("Failed to read 'access_flags'");
	let (input, name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_index'");
	let (input, descriptor_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'descriptor_index'");
	
	let (input, attributes) = length_count(be_u16, attribute_info_parser::<'a, E>)(input).expect("Failed to read 'attributes_count' or 'attributes'");
	let attributes_count = attributes.len() as U2;

	Ok((input, FieldInfo {
		access_flags,
		name_index,
		descriptor_index,
		attributes_count,
		attributes,
	}))
}