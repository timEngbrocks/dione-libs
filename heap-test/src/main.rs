use std::{thread::sleep, time::Duration};

use heap::vm_heap::Heap;
use rand::Rng;

const WAIT_SECONDS: u64 = 0;
const DATA_SIZE: usize = 100 * 1024 * 1024;
const ITERATIONS: usize = 10;

struct BigData {
    data: Vec<u8>,
}
impl BigData {
    pub fn construct() -> BigData {
        let mut data = Vec::with_capacity(DATA_SIZE);
        for _ in 0..DATA_SIZE {
            data.push(rand::thread_rng().gen())
        }
        BigData {
            data,
        }
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
impl Clone for BigData {
    fn clone(&self) -> Self {
        panic!("Should not clone this struct!");
    }
}

fn main() {
    sleep(Duration::from_secs(WAIT_SECONDS));

    let mut ptrs = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let x = BigData::construct();
    
        let ptr = Heap::it().add(x);
        ptrs.push(ptr);
    }

    println!("Finished construction!");

    for i in 0..ITERATIONS {
        let ptr = ptrs.get(i).unwrap();
        let y = Heap::it().get::<BigData>(ptr);
        assert!(y.is_some());
        let y = y.unwrap();
        assert!(y.data().len() == DATA_SIZE);
    }

    println!("Finished 1st get!");

    for i in 0..ITERATIONS {
        let ptr = ptrs.get(i).unwrap();
        let updated_x = BigData::construct();
        Heap::it().write(ptr, updated_x);
    }

    println!("Finished mutation!");

    for i in 0..ITERATIONS {
        let ptr = ptrs.get(i).unwrap();
        let y = Heap::it().get::<BigData>(ptr);
        assert!(y.is_some());
        let y = y.unwrap();
        assert!(y.data().len() == DATA_SIZE);
    }

    println!("Finished 2nd get!");

    for ptr in ptrs {
        Heap::it().remove(ptr);

        let z = Heap::it().get::<BigData>(&ptr);
        assert!(z.is_none());
    }

    println!("Finished free!");

    sleep(Duration::from_secs(WAIT_SECONDS));
}
