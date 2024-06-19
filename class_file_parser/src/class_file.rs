use nom::{combinator::map, error::ParseError, multi::length_count, number::complete::{be_u16, be_u32}, IResult};

use crate::{attribute_info::{attribute_info_parser, AttributeInfo}, cp_info::{cp_info_parser, CPInfo}, field_info::{field_info_parser, FieldInfo}, method_info::{method_info_parser, MethodInfo}, U2, U4};

#[derive(Debug)]
pub struct ClassFile {
	pub magic: U4,
    pub minor_version: U2,
    pub major_version: U2,
    pub constant_pool_count: U2,
    pub constant_pool: Vec<CPInfo>,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    pub interfaces_count: U2,
    pub interfaces: Vec<U2>,
    pub fields_count: U2,
    pub fields: Vec<FieldInfo>,
    pub methods_count: U2,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: U2,
    pub attributes: Vec<AttributeInfo>,
}

pub fn class_file_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&[u8], ClassFile> {
	let (input, magic) = be_u32::<&[u8], E>(input).expect("Failed to read 'magic'");
	let (input, minor_version) = be_u16::<&[u8], E>(input).expect("Failed to read 'minor_version'");
	let (input, major_version) = be_u16::<&[u8], E>(input).expect("Failed to read 'major_version'");
	
	let (input, constant_pool) = length_count(map(be_u16, |i| {
		i - 1
	}), cp_info_parser::<'a, E>)(input).expect("Failed to read 'constant_pool_count' or 'constant_pool'");
	let constant_pool_count = constant_pool.len() as U2;

	let (input, access_flags) = be_u16::<&[u8], E>(input).expect("Failed to read 'access_flags'");
	let (input, this_class) = be_u16::<&[u8], E>(input).expect("Failed to read 'this_class'");
	let (input, super_class) = be_u16::<&[u8], E>(input).expect("Failed to read 'super_class'");

	let (input, interfaces) = length_count(be_u16::<&[u8], E>, be_u16::<&[u8], E>)(input).expect("Failed to read 'interfaces_count' or 'interfaces'");
	let interfaces_count = interfaces.len() as U2;
	let (input, fields) = length_count(be_u16, field_info_parser::<'a, E>)(input).expect("Failed to read 'fields_count' or 'fields'");
	let fields_count = fields.len() as U2;
	let (input, methods) = length_count(be_u16, method_info_parser::<'a, E>)(input).expect("Failed to read 'methods_count' or 'methods'");
	let methods_count = methods.len() as U2;
	let (input, attributes) = length_count(be_u16, attribute_info_parser::<'a, E>)(input).expect("Failed to read 'attributes_count' or 'attributes'");
	let attributes_count = attributes.len() as U2;

	Ok((input, ClassFile {
		magic,
		minor_version,
		major_version,
		constant_pool_count,
		constant_pool,
		access_flags,
		this_class,
		super_class,
		interfaces_count,
		interfaces,
		fields_count,
		fields,
		methods_count,
		methods,
		attributes_count,
		attributes,
	}))
}