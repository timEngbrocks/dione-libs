use std::fmt::Display;

use boolean::Boolean;
use byte::Byte;
use char::Char;
use double::Double;
use float::Float;
use int::Int;
use long::Long;
use reference::Reference;
use return_address::ReturnAddress;
use short::Short;

pub mod byte;
pub mod short;
pub mod int;
pub mod long;
pub mod char;
pub mod float;
pub mod double;
pub mod boolean;
pub mod return_address;
pub mod reference;

pub enum Types {
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
    Char(Char),
    Float(Float),
    Double(Double),
    Boolean(Boolean),
    ReturnAddress(ReturnAddress),
    Reference(Reference),
}

pub enum PrimitiveTypes {
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
    Char(Char),
    Float(Float),
    Double(Double),
    Boolean(Boolean),
    ReturnAddress(ReturnAddress),
}

pub enum IntegralTypes {
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
    Char(Char),
}

pub enum FloatingPointTypes {
    Float(Float),
    Double(Double),
}

pub enum ReferenceTypes {
    Reference(Reference),
}

pub trait Type {
    type T;

    fn new() -> Self;
    fn from_value(value: Self::T) -> Self;
    fn get(&self) -> &Self::T;
    fn set(&mut self, value: Self::T);
    fn width(&self) -> u16;
    fn to_string(&self) -> String;
}

impl Clone for Types {
    fn clone(&self) -> Self {
        match self {
            Self::Byte(v) => Self::Byte(Byte::from_value(*v.get())),
            Self::Short(v) => Self::Short(Short::from_value(*v.get())),
            Self::Int(v) => Self::Int(Int::from_value(*v.get())),
            Self::Long(v) => Self::Long(Long::from_value(*v.get())),
            Self::Char(v) => Self::Char(Char::from_value(*v.get())),
            Self::Float(v) => Self::Float(Float::from_value(*v.get())),
            Self::Double(v) => Self::Double(Double::from_value(*v.get())),
            Self::Boolean(v) => Self::Boolean(Boolean::from_value(*v.get())),
            Self::ReturnAddress(v) => Self::ReturnAddress(ReturnAddress::from_value(*v.get())),
            Self::Reference(v) => Self::Reference(Reference::from_value(*v.get())),
        }
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(v) => write!(f, "{}", Byte::to_string(v)),
            Self::Short(v) => write!(f, "{}", Short::to_string(v)),
            Self::Int(v) => write!(f, "{}", Int::to_string(v)),
            Self::Long(v) => write!(f, "{}", Long::to_string(v)),
            Self::Char(v) => write!(f, "{}", Char::to_string(v)),
            Self::Float(v) => write!(f, "{}", Float::to_string(v)),
            Self::Double(v) => write!(f, "{}", Double::to_string(v)),
            Self::Boolean(v) => write!(f, "{}", Boolean::to_string(v)),
            Self::ReturnAddress(v) => write!(f, "{}", ReturnAddress::to_string(v)),
            Self::Reference(v) => write!(f, "{}", Reference::to_string(v)),
        }
    }
}