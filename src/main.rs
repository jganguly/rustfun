mod mod_example;                    // mod_example is the name of the diretory containing a mod 
                                    // a file with the name of mod and .rs extension has the function definition

use crate::mod_example::sub;

fn main() {
    println!("Hello, world!");
    sub::f1();


}
