use std::process;

#[derive(Debug)]
struct Sentence<'a> {
	part: &'a str
}

pub fn split(sentence: &str) -> Vec<&str> {
    let sent = sentence.split(".");
    let mut vec: Vec<&str> = sent.collect();
	vec
}


pub fn vec_example() {
	let mut vec: Vec<u32> = vec![1, 2, 3];

	vec.insert(1,52);			// insert element at index
	println!("{:?}",vec);

	let elem = vec.remove(1);	// remove element at index
	println!("{:?} {:?}",elem,vec);

	vec[1] = 9;					// modify element at position
	println!("{:?}",vec);

	vec.push(99);				// push
	println!("{:?}",vec);	

	vec.pop();					// pop
	println!("{:?}",vec);

	let index = vec.iter().position(|x| x == &9).unwrap();	// remove element
	vec.remove(index);
	println!("{:?}",vec);


	let mut vec2: Vec<&str> = vec!["Hello", "how","are","you"];
	println!("{:?}",vec2);
	for v in vec2.iter() {
		println!("{:?}",v);
	}


	let mut vec3: Vec<String> = vec![];

	for v in vec2.iter() {
	    let r = v.to_ascii_uppercase();
	    vec3.push(r);
	}

	println!("{:?}",vec3);

	// A unit struct
	struct Unit;
	let unit = Unit;

    process::exit(0x0100);
}

