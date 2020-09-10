use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;
use std::fs;

pub fn std_inp() {
	let mut line = String::new();
	println!("Please enter your name:");
	let nb = std::io::stdin().read_line(&mut line).unwrap();
	println!("Hi {}", line);
	println!("# of bytes read , {}", nb);
}

pub fn std_out() {
	let b1 = std::io::stdout()
		.write("Hi ".as_bytes()).unwrap();
	let b2 = std::io::stdout()
		.write(String::from("There\n").as_bytes()).unwrap();
	std::io::stdout().
		write(format!("#bytes written {}",(b1+b2))
			.as_bytes()).unwrap();
}

pub fn cl_arg() {
	let cmd_line = std::env::args();
	println!("# of command line arguments:{}",cmd_line.len());
	for arg in cmd_line {
		println!("{}",arg); 
	}
}

pub fn file_read(filename: &str){
	let mut file = std::fs::File::open(filename).unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
}

pub fn file_write(filename: &str, s: &str) {
	let mut file = std::fs::File::create(filename)
		.expect("Create failed");
	file.write_all(s.as_bytes())
		.expect("write failed");
	println!("Write completed" );
}

pub fn file_append(filename: &str, s: &str) {
	let mut file = OpenOptions::new()
		.append(true).open(filename)
		.expect("Failed to open file");
	file.write_all(s.as_bytes()).expect("write failure");
	println!("Appended file {}",filename);
}

pub fn file_copy(src: &str, des: &str) {
	let mut file_inp = std::fs::File::open(src).unwrap();
	let mut file_out = std::fs::File::create(des).unwrap();
	let mut buffer = [0u8; 4096];
	loop {
		let nbytes = file_inp.read(&mut buffer).unwrap();
		file_out.write(&buffer[..nbytes]).unwrap();
		if nbytes < buffer.len() { 
			break; 
		}
	}
}

pub fn file_delete(filename: &str) {
	fs::remove_file(filename).expect("Unable to delete file");
	println!("Deleted file {}",filename);
}

