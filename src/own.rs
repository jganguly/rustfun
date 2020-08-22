pub fn scope_example() {
    {
        let x = 1;
        println!("x: {}", x);
    }

    // println!("x: {}", x); // ERROR
}

pub fn move_example() {
	let a = vec![1, 2, 3];  // a growable array literal
	let b = a;              // a can no longer be used
	println!("a: {:?}", b); 
	// println!("a: {:?}", a); // ERROR
}

pub fn move_example2() {
    let v = vec![1,2,3];
    let s = sum(v);
	// println!("sum of {:?}: {}", v, s); // ERROR
}



fn sum(vector: Vec<i32>) -> i32 {
	let mut sum = 0;

	for item in vector {
		sum = sum + item;
    }
    
	sum
}

pub fn create_series(x: i32) -> Vec<i32> {
	let result = vec![x, x+1, x+2, x+3, x+4];
	result
}


pub fn copy_trait_example() {
	let a = 42;
    let b = 94;
    let c = a + b;
    println!("The sum of {} and {} is {}", a, b, c); // NO ERROR
}


pub fn struct_copy_example() {
    #[derive(Debug,Clone,Copy)]
    struct Person {
        age: i8
    }

    let alice = Person { age: 42 };
	let bob = alice;

	println!("alice: {:?}\nbob: {:?}", alice, bob);
}


pub fn ref_example() {
    let s = String::from("hello");
	let len = calculate_length(&s);
	println!("The length of '{}' is {}.", s, len);	// no error
}

fn calculate_length(s: &String) -> usize {
	s.len()
}


pub fn mut_ref_example() {
	let mut s = String::from("Hello");
    change(&mut s);
    println!("{}",s);
}

fn change(some_string: &mut String) {
	some_string.push_str(" world!");
}

/*
pub fn mut_ref_restrict() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    
    // ERROR: will not compile    
    // cannot borrow 's' as mutable more than once at a time
    println!("{}, {}", r1, r2); 
}

pub fn mut_ref_restrict2() {
    let mut s = String::from("hello");

    // ERROR: will not compile    
    // cannot borrow 's' as mutable because it is also borrowed as immutable.
    let r1 = &s;     // no problem
    let r2 = &mut s; // problem
    
    println!("{}, {}", r1, r2);
} 
*/


pub fn no_dang_ref_example() {
	let reference_to_nothing = no_dangle();
}

/* ERROR: 
fn dangle() -> &String {    // This will not compile
	let s = String::from("hello");
	&s
}
*/

fn no_dangle() -> String {
	let s = String::from("hello");
	s
}

/* This code will not compile
pub fn lifetime_example(x: &str, y: &str) -> &str { // Error
    if x.bytes().len() > y.bytes().len() {
        x
    } else {
        y
    }
}
*/

pub fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.bytes().len() > y.bytes().len() {
        x
    } else {
        y
    }
}

