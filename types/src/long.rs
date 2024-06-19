use crate::Type;

pub struct Long {
	value: i64,
}

impl Type for Long {
	type T = i64;
	
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
		2
	}
	
	fn to_string(&self) -> String {
		format!("long({})", self.value)
	}
}