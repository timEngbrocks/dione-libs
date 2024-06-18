use std::{collections::HashSet, mem, sync::{Mutex, OnceLock}};

use libc::{c_void, free, malloc, size_t};

pub struct Heap {
	allocated: HashSet<usize>,
	lock: Mutex<()>,
}
pub static mut HEAP: OnceLock<Heap> = OnceLock::new();
impl Heap {
	pub fn it() -> &'static mut Self {
		unsafe {
			HEAP.get_or_init(|| {
				Heap {
					allocated: HashSet::new(),
					lock: Mutex::new(()),
				}
			});
			match HEAP.get_mut() {
				Some(x) => x,
				None => panic!("Failed to construct Heap"),
			}
		}
	}

	pub fn add<T>(&mut self, data: T) -> usize
	where
		T: Send + Sync + Sized + 'static
	{
		unsafe {
			let guard = self.lock.lock().expect("Could not acquire lock");
			let ptr = malloc(mem::size_of_val(&data) as size_t) as *mut T;
			if ptr.is_null() {
				panic!("Failed to allocate memory");
			}
			ptr.write(data);
			let addr = ptr.addr();
			self.allocated.insert(addr);
			drop(guard);
			addr
		}
	}

	pub fn get<T>(&self, ptr: &usize) -> Option<T>
	where
		T: Sized + 'static
	{
		let guard = self.lock.lock().expect("Could not acquire lock");
		if !self.allocated.contains(ptr) {
			return None;
		}
		let ptr = *ptr as *mut c_void;
		if ptr.is_null() {
			return None;
		}
		let result = unsafe { Some((ptr as *mut T).read()) };
		drop(guard);
		result
	}

	pub fn write<T>(&mut self, ptr: &usize, data: T)
	where
		T: Send + Sync + Sized + 'static
	{
		let guard = self.lock.lock().expect("Could not acquire lock");
		if !self.allocated.contains(ptr) {
			panic!("Tried to write to unallocated pointer");
		}
		let ptr = *ptr as *mut c_void;
		if ptr.is_null() {
			panic!("Tried to write to null pointer");
		}
		unsafe { (ptr as *mut T).write(data); }
		drop(guard);
	}

	pub fn remove(&mut self, ptr: usize) {
		let guard = self.lock.lock().expect("Could not acquire lock");
		if !self.allocated.contains(&ptr) {
			panic!("Tried to free unallocated pointer");
		}
		let ptr_to_remove = ptr;
		let ptr = ptr as *mut c_void;
		if ptr.is_null() {
			panic!("Tried to free null pointer");
		}
		self.allocated.remove(&ptr_to_remove);
		unsafe { free(ptr); }
		drop(guard);
	}
}

#[cfg(test)]
mod tests {
	use std::{sync::mpsc::{channel, Receiver, Sender}, thread};
	use super::*;

	struct SomeData {
		a: usize,
		b: usize,
	}

	const ITERATIONS: usize = 1_000_000;

	#[test]
	fn base() {
		let a = 100;
		let b = 500;

		let x = SomeData {
			a,
			b,
		};
	
		let ptr = Heap::it().add(x);
		let y = Heap::it().get::<SomeData>(&ptr);
		assert!(y.is_some());
		let y = y.unwrap();
		let result = y.a + y.b;
		assert!(result == a + b);

		let updated_x = SomeData {
			a: a * 2,
			b: b * 2,
		};
		Heap::it().write(&ptr, updated_x);

		let y = Heap::it().get::<SomeData>(&ptr);
		assert!(y.is_some());
		let y = y.unwrap();
		let result = y.a + y.b;
		assert!(result == a * 2 + b * 2);

		Heap::it().remove(ptr);

		let z = Heap::it().get::<SomeData>(&ptr);
		assert!(z.is_none());
	}

	#[test]
	fn multi() {
		let mut ptrs = Vec::with_capacity(ITERATIONS);
		for i in 0..ITERATIONS {
			let x = SomeData {
				a: i,
				b: i,
			};
		
			let ptr = Heap::it().add(x);
			ptrs.push(ptr);
		}

		for i in 0..ITERATIONS {
			let ptr = ptrs.get(i).unwrap();
			let y = Heap::it().get::<SomeData>(ptr);
			assert!(y.is_some());
			let y = y.unwrap();
			let result = y.a + y.b;
			assert!(result == i + i);
		}

		for i in 0..ITERATIONS {
			let ptr = ptrs.get(i).unwrap();
			let updated_x = SomeData {
				a: i * 2,
				b: i * 2,
			};
			Heap::it().write(ptr, updated_x);
		}

		for i in 0..ITERATIONS {
			let ptr = ptrs.get(i).unwrap();
			let y = Heap::it().get::<SomeData>(ptr);
			assert!(y.is_some());
			let y = y.unwrap();
			let result = y.a + y.b;
			assert!(result == i * 2 + i * 2);
		}

		for ptr in ptrs {
			Heap::it().remove(ptr);

			let z = Heap::it().get::<SomeData>(&ptr);
			assert!(z.is_none());
		}
	}

	#[test]
	fn thread_base() {
		let (tx_parent, rx_child): (Sender<usize>, Receiver<usize>) = channel();
		let (tx_child, rx_parent): (Sender<bool>, Receiver<bool>) = channel();
		let child = thread::spawn(move || {
			let child_ptr = match rx_child.recv() {
				Ok(ptr) => ptr,
				Err(e) => panic!("Failed to recv ptr from parent! Got: '{:?}'", e),
			};

			let y = Heap::it().get::<SomeData>(&child_ptr);
			assert!(y.is_some());
			let y = y.unwrap();
			let result = y.a + y.b;
			tx_child.send(result == 600).expect("Failed to send sync to parent");

			let child_ptr = match rx_child.recv() {
				Ok(ptr) => ptr,
				Err(e) => panic!("Failed to recv ptr from parent! Got: '{:?}'", e),
			};
			let y = Heap::it().get::<SomeData>(&child_ptr);
			assert!(y.is_some());
			let y = y.unwrap();
			let result = y.a + y.b;
			tx_child.send(result == 1200).expect("Failed to send sync to parent");

			let child_ptr = match rx_child.recv() {
				Ok(ptr) => ptr,
				Err(e) => panic!("Failed to recv ptr from parent! Got: '{:?}'", e),
			};
			let z = Heap::it().get::<SomeData>(&child_ptr);
			tx_child.send(z.is_none()).expect("Failed to send sync to parent");
		});

		let a = 100;
		let b = 500;

		let x = SomeData {
			a,
			b,
		};
	
		let ptr = Heap::it().add(x);
		tx_parent.send(ptr).expect("Failed to send ptr to child");
		assert!(rx_parent.recv().expect("Failed to recv sync from child"));

		let updated_x = SomeData {
			a: a * 2,
			b: b * 2,
		};
		Heap::it().write(&ptr, updated_x);
		tx_parent.send(ptr).expect("Failed to send ptr to child");
		assert!(rx_parent.recv().expect("Failed to recv sync from child"));

		Heap::it().remove(ptr);
		tx_parent.send(ptr).expect("Failed to send ptr to child");
		assert!(rx_parent.recv().expect("Failed to recv sync from child"));

		child.join().expect("Child panicked!");
	}

	#[test]
	fn thread_multi() {
		let (tx_parent, rx_child): (Sender<Vec<usize>>, Receiver<Vec<usize>>) = channel();
		let (tx_child, rx_parent): (Sender<bool>, Receiver<bool>) = channel();
		let child = thread::spawn(move || {
			let child_ptrs = match rx_child.recv() {
				Ok(ptr) => ptr,
				Err(e) => panic!("Failed to recv ptrs from parent! Got: '{:?}'", e),
			};
			let mut success = true;
			for i in 0..ITERATIONS {
				let ptr = child_ptrs.get(i).unwrap();
				let y = Heap::it().get::<SomeData>(ptr);
				assert!(y.is_some());
				let y = y.unwrap();
				let result = y.a + y.b;
				success &= result == i + i;
			}
			tx_child.send(success).expect("Failed to send sync to parent");

			let child_ptrs = match rx_child.recv() {
				Ok(ptr) => ptr,
				Err(e) => panic!("Failed to recv ptrs from parent! Got: '{:?}'", e),
			};
			let mut success = true;
			for i in 0..ITERATIONS {
				let ptr = child_ptrs.get(i).unwrap();
				let y = Heap::it().get::<SomeData>(ptr);
				assert!(y.is_some());
				let y = y.unwrap();
				let result = y.a + y.b;
				success &= result == i * 2 + i * 2;
			}
			tx_child.send(success).expect("Failed to send sync to parent");

			let child_ptrs = match rx_child.recv() {
				Ok(ptr) => ptr,
				Err(e) => panic!("Failed to recv ptrs from parent! Got: '{:?}'", e),
			};
			let mut success = true;
			for i in 0..ITERATIONS {
				let ptr = child_ptrs.get(i).unwrap();
				let z = Heap::it().get::<SomeData>(ptr);
				success &= z.is_none();
			}
			tx_child.send(success).expect("Failed to send sync to parent");
		});

		let mut ptrs = Vec::with_capacity(ITERATIONS);
		for i in 0..ITERATIONS {
			let x = SomeData {
				a: i,
				b: i,
			};
		
			let ptr = Heap::it().add(x);
			ptrs.push(ptr);
		}
		tx_parent.send(ptrs.clone()).expect("Failed to send ptrs to child");
		assert!(rx_parent.recv().expect("Failed to recv sync from child"));

		for i in 0..ITERATIONS {
			let ptr = ptrs.get(i).unwrap();
			let updated_x = SomeData {
				a: i * 2,
				b: i * 2,
			};
			Heap::it().write(ptr, updated_x);
		}
		tx_parent.send(ptrs.clone()).expect("Failed to send ptr to child");
		assert!(rx_parent.recv().expect("Failed to recv sync from child"));

		for ptr in ptrs.clone() {
			Heap::it().remove(ptr);

			let z = Heap::it().get::<SomeData>(&ptr);
			assert!(z.is_none());
		}
		tx_parent.send(ptrs.clone()).expect("Failed to send ptr to child");
		assert!(rx_parent.recv().expect("Failed to recv sync from child"));

		child.join().expect("Child panicked!");
	}
}