pub fn concept() {
	let x = 5;
	let mut y = 6;
	y = y + 1;
	const MAX_POINTS: u32 = 100_000*100;
	println!("x={} y={} MAX_POINTS={}",x,y,MAX_POINTS);
}

pub fn shadow() {
    let x = 5;
	let x = x + 1;
    let x = x * 2;
	
	let spaces = "   ";
    let spaces = spaces.len();
    
    println!("{} {}", x,spaces);
}

pub fn data_types() {
	let x = 3; // Type inference will assign x to be of type i32
    let num: i32 = "142".parse().expect("Not a number!");
    let y: f64 = 300.43;
    let c = 'A';
    let s = "hello";
    let t = true;
    
	println!("{} {} {} {} {} {}", x, num, y, c, s, t);
}

pub fn tuple_example() {
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (_, y, _) = tup;
	println!("The value of y is: {}", y);
}

pub fn array_example() {
	let a = [1, 2, 3, 4, 5];
	let b = [3; 5];				   // 5 is the size of the array
    let c: [i32; 5] = [1,2,3,4,5]; // type is stated with semicolon
    println!("a={:?}\nb={:?}\nc={:?}", a,b,c);
}

pub fn function_example(a: i32, b:i32) -> i32 {
	let c = a + b;
	c
}

pub fn ctrlflow_example() {
    let a = 12;

    if a < 10 {
        println!("low number");
    }
    else if (a > 10) && (a < 20) {
        println!("moderate number");
    }
    else {
        println!("high number");
    }
}

pub fn loop_example() {
    let arr = [10, 20, 30, 40, 50];

    for elem in arr.iter() {
        println!("Element is: {}", elem);   
    }

    let mut i = 0;
    loop {
        println!("{} {}", i, arr[i]);  
        i = i + 1;
        if (i == arr.len()) {
            break;
        }
    }
}

pub fn match_example() {
    let x = 100;

    match (x) {
        100 => {
            println!("Hit a century {:?}", x);
        },
        200 => {
            println!("Hit a double century {:?}", x);
        },
        _ => ()
    }
}