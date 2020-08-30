#[derive(Debug)]	
struct MyBox<T> { // same as: struct MyBox<T>(T);
	a: T
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.a
	}
}

pub fn smaptr_example() {
    let x = MyBox{a : 111};  
    println!("Derefenced: {}",*(x.deref())); 
}


struct mysmaptr { 
    data: String
}

impl Drop for mysmaptr {

    fn drop(&mut self) {
        println!("Dropping struct mysmaptr with data {}", self.data);
    }
}

pub fn smaptr_example2() {
    let x = mysmaptr{ data : String::from("Hello") };
    println!("struct mysmaptr with data {}", x.data);
}