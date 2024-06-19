use crate::Type;

pub struct Boolean {
	value: bool,
}

impl Type for Boolean {
	type T = bool;
	
	fn new() -> Self {
		Self {
			value: false,
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
		format!("boolean({})", self.value)
	}
}