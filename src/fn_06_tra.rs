pub trait Animal {
    fn eat(&self) {
        println!("I eat grass");
    }
}

pub struct Herbivore;

impl Animal for Herbivore{
    fn eat(&self) {
        println!("I eat plants");
    }
}

pub struct Carnivore;

impl Animal for Carnivore {
    fn eat(&self) {
        println!("I eat meat");
    }
}



pub trait Activity {
    fn fly(&self);
}

#[derive(Debug)]
pub struct Eagle;

impl Activity for Eagle {
    fn fly(&self) {
        println!("{:?} is flying",&self);
    }
}

pub fn activity<T: Activity + std::fmt::Debug>(bird: T) { 
    println!("I fly as an {:?}",bird);
}

pub struct Hen;


#[derive(Debug)]
pub struct Horse;

#[derive(Debug)]
pub struct Deer;

#[derive(Debug)]
pub struct Tiger;

#[derive(Debug)]
pub struct Duck;

pub trait Sound {
    fn sound(&self);
}

impl Sound for Horse {
    fn sound(&self) {
        println!("{:?} neighs",&self)
    }
}

impl Sound for Deer {
    fn sound(&self) {
        println!("{:?} barks",&self)
    }
}

impl Sound for Tiger {
    fn sound(&self) {
        println!("{:?} roars",&self)
    }
}

impl Sound for Duck {
    fn sound(&self) {
        println!("{:?} quacks",&self)
    }
}

pub struct SoundBook {
    pub sounds: Vec<Box<dyn Sound>>
}

impl SoundBook {

    pub fn run(&self) {
        for s in self.sounds.iter() {
            s.sound();
        }
    }
}



// use std::fmt::Debug;
// fn print_it( input: impl Debug + 'static ) {
//     println!( "'static value passed in is: {:?}", input );
// }

// pub fn traitbound_example() {
//     // i is owned and contains no references, thus it's 'static:
//     let i = 5;
//     print_it(i);

//     // ERROR, &i only has the lifetime defined by the scope of traitbound_example(), not 'static:
//     // print_it(&i);
// }
