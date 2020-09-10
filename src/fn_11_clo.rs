pub fn closure_example1(x: i32, y: i32) {
    let add = |x,y| {
        x + y
    };
    println!("Closure {:?}", add(x,y));
}

pub fn closure_example2(x:i32) {
    let y = 3;
    let add = |x| {
        x + y
    };
    println!("Closure {:?}", add(x));
}

pub fn closure_example3(x:i32) {
    let y = 3;
    let add = |x| {
        x + y
    };
    println!("Receives Closure {:?}", add(5));
}

fn receive_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(4);
    println!("closure => {}", result);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,	// trait bound
{
    calc: T,			// calc stores the closure that is trait bound
    value: Option<u32>,	// Result of calling the function calc
}

impl<T> Cacher<T>
where 
	T: Fn(u32) -> u32 , // trait bound
{
	fn new(calc: T) -> Cacher<T> {
		Cacher {		// expression returning the function
			calc,
			value: None
		}
	} 

	fn func(&mut self, arg: u32) -> u32 {
		match self.value {
			Some(v) => v, // value exists, return v
			None => {	  // value does not exit
				let v = (self.calc)(arg);	// invoke calc with arg
				self.value = Some(v);		// wrap value in Option
				v							// return v
			}
		}
	}
}

use std::thread;
use std::time::Duration;
pub fn generate_force(hp: u32, random_number: u32) {

    let mut my_closure = Cacher::new(|number| {
        println!("calculating HP ...");
        thread::sleep(Duration::from_secs(1));
        number
    });

    if hp < 25 {
        println!("Low HP drive slow {}", my_closure.func(hp));
        println!("Low HP drive steady {}", my_closure.func(hp));
    } else {
    
        if random_number == 3 {
            println!("No HP generated");
        } else {
            println!(
                "Sufficient HP {}", my_closure.func(hp)
            );
        }
    }

}


