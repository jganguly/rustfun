pub fn panic_example() {
    let x = 5;
    if (x > 10) {
        panic!("I am panicking, can't proceed any further");
    }
    println!("I won't print this");
}

use std::fs::File;
pub fn error_example() {

    let f = File::open("/Users/jaideep.ganguly/rust/src/inp2.txt").expect("File not found");   // file doesn't exist
    // let f = File::open("/Users/jaideep.ganguly/rust/src/inp.txt");   // file doesn't exist
    
    // match f {
    //    Ok(f)=> {
    //       println!("file: {:?}",f);
    //    },
    //    Err(e)=> {
    //       println!("file not found{:?}",e);   //handled error
    //    }
    // }
    println!("I will print this");
 }