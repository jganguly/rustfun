pub struct Rect {
    pub w: i32,
    pub h: i32
}

impl Rect {
    pub fn area(&self) -> i32 {
        self.w * self.h
    }
}

pub struct Function;

impl Function {
    pub fn say_hello(&self) {
        println!("{:?}", "Hello" );
    }
}


#[derive(Debug)]
pub struct Person<'a> {
    pub name: &'a str,
    pub id:       i32,
    pub amt:      f64
}

impl<'a> Person<'a> {
    // pub fn set_id(&mut self, num: &mut i32) {
    pub fn set_id(&mut self, num: &'a i32) {        
        println!("{}",num);
        
        let mut x: i32;
        x = *num + 1;
        println!("{}",x);

        self.id = x;
    }
}








