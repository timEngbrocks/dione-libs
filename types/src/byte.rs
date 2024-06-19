use crate::Type;

pub struct Byte {
	value: i8,
}

impl Type for Byte {
	type T = i8;
	
	fn new() -> Self {
		Self {
			value: 0,
		}
	}
	
	fn from_value(value: Self::T) -> Self {
		Self {
			value,
		}
	}
	
	fn get(&self) -> &Self::T {
		&self.value
	}
	
	fn set(&mut self, value: Self::T) {
		self.value = value;
	}
	
	fn width(&self) -> u16 {
		1
	}
	
	fn to_string(&self) -> String {
		format!("byte({})", self.value)
	}
}