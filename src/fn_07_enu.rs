#[derive(Debug)]
pub enum Gender {
	Male,
	Female
}

#[derive(Debug)]
pub struct MyBlack {
    pub name: String,
    pub rgb: (u8,u8,u8)
}

#[derive(Debug)]
pub enum Color {
    Black(MyBlack),
    White(u8,u8,u8)
}

impl Color {
    pub fn printColor(&self) { 
        println!("Hi!");
    }
}

pub fn match_example(number: i32) {

    match is_even(number) {
       Some(data) => {
          if data == true {
             println!("Even number");
          }
       },
       None => {
          println!("Not an even number");
       }
    }
 }
 
 fn is_even(number:i32) -> Option<bool> {
    if number%2 == 0 {
       Some(true)
    } else {
       None
    }
 }