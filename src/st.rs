pub struct Function;    // unit struct, has no data

impl Function {
    pub fn say_hello(&self) {
        println!("{:?}", "Hello" );
    }
}

pub struct Rect<'a> {
    pub id: &'a str,
    pub width: i32,
    pub length: i32
}

impl<'a> Rect<'a> {
    pub fn area(&self) -> i32 {
        self.width * self.length
    }

    pub fn volume(&self, height: i32) -> i32 {
        self.area()*height
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








