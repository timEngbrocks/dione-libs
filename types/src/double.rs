use crate::Type;

pub struct Double {
	value: f64,
}

impl Type for Double {
	type T = f64;
	
	fn new() -> Self {
		Self {
			value: 0.0,
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
		format!("double({})", self.value)
	}
}