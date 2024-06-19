use crate::Type;

pub struct Char {
	value: u16,
}

impl Type for Char {
	type T = u16;
	
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
		format!("char({})", self.value)
	}
}