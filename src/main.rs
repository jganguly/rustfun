#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// mod_org is the name of the folder; it has a file named mod.rs
mod mod_org;     
// mod_example.rs file is in "src" directory; contains mod named test
mod mod_example;  

mod fn_03_basic;    // basic
mod fn_04_own;      // ownership
mod fn_05_st;       // struct
mod fn_06_tra;      // trait
mod fn_07_enu;      // enum
mod fn_08_col;      // collection
mod fn_09_err;      // error handling
mod fn_10_io;       // input and output
mod fn_11_clo;      // closure
mod fn_12_sma;      // smart pointers
mod fn_13_con;      // concurrency
mod fn_14_util;      // utilities

mod app_db;


use mod_example::mod_test;
use mysql::prelude::TextQuery;


fn main() {
    
    /* basic concepts */
    // fn_03_basic::concept();
    // fn_03_basic::shadow();
    // fn_03_basic::data_types();
    // fn_03_basic::tuple_example();
    // fn_03_basic::array_example();
    // fn_03_basic::ctrlflow_example();
    // fn_03_basic::loop_example();


    /* mod example, organizing code */

    // mod_org::f1();
    // mod_example::mod_test::f2();

    /* own */
    // fn_04_own::example1();
    // fn_04_own::example2();
    // fn_04_own::example3();

    // let y = fn_04_own::example4(3);
    // println!("{:?}",y );

    // let mut x = 10;
    // fn_04_own::example5(&mut x);
    // println!("{:?}", x);
    // let y = fn_04_own::example6(&mut x);
    // println!("{:?}", y);
    // let s = fn_04_own::example7("Hello");
    // println!("{:?}", s);
    // fn_04_own::copy_trait_example();
    // fn_04_own::struct_copy_example();

    let mut x: i32 = 42;
    fn_04_own::example5(&mut x);  // x is passed by reference
    println!("{}\n",x);

    let mut x: i32 = 42;
    let y = fn_04_own::example6(&mut x);
    println!("Returned value {:?}\n",y);

    let mut s = "Hello";
    let s = fn_04_own::example7(&mut s);
    println!("Returned slice {:?}\n",s);

    // fn_04_own::copy_trait_example();
    // fn_04_own::struct_copy_example();
    // fn_04_own::ref_example();
    // fn_04_own::mut_ref_example();

    // fn_04_own::mut_ref_restrict();
    // fn_04_own::mut_ref_restrict2();

    // let alice = "Alice";
    // let bob = "Bob";
    // println!("{}", fn_04_own::lifetime_example(alice, bob));

    /* st */
    let f = fn_05_st::Function{};
    f.say_hello();

    let r = fn_05_st::Rect {id:"abc",width:10, length:20};
    println!("Area = {}",r.area());
    println!("Volume = {}",r.volume(10));

    let mut person = 
        fn_05_st::Person { 
            name: "Tim",
            id: 0, 
            amt: 52.54
        };
    let id = 900;
    person.set_id(&id);
    println!("{:?}", person);

    /* tra */
    use fn_06_tra::Animal;
    let h = fn_06_tra::Herbivore;
    h.eat();

    let c = fn_06_tra::Carnivore;
    c.eat();

    use fn_06_tra::Activity;
    let eagle = fn_06_tra::Eagle;
    eagle.fly();
    fn_06_tra::activity(eagle);

    // let hen = fn_06_tra::Hen;
    // fn_06_tra::activity(hen);      // Error

    let sound_book = fn_06_tra::SoundBook {
        sounds: vec! [
            Box::new(fn_06_tra::Horse{}),
            Box::new(fn_06_tra::Deer{}),
            Box::new(fn_06_tra::Tiger{}),
            Box::new(fn_06_tra::Duck{})
        ]
    };

    sound_book.run();    

    /* enu */
    use fn_07_enu::Gender;
    let male = fn_07_enu::Gender::Male;
    let female = fn_07_enu::Gender::Female;
    println!("{:?}",male);
    println!("{:?}",female);
    

    let my_black = fn_07_enu::MyBlack {
        name: String::from("my black"), 
        rgb: (10,10,10)
    };
    let black = fn_07_enu::Color::Black(my_black);
    let white = fn_07_enu::Color::White(255,255,255);
    println!("{:?}",black);
    println!("{:?}",white);
    white.printColor();

    let s = Some(803);
    assert_eq! (s.is_some(), true);
    println!("{}",s.unwrap());

    match black {
        fn_07_enu::Color::White(x,y,z) => println!("{} {} {}",x,y,z),
        fn_07_enu::Color::Black(x) => println!("{:?}",x.rgb),
    }

    let some_u8_value = 4u8;
    match some_u8_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        9 => println!("Nine"),
        _ => (),
    }
    
    fn_07_enu::match_example(4);


    /* col */
    fn_08_col::vec_example();
    fn_08_col::hm_example();
    fn_08_col::hs_example();

    /* err */
    // fn_09_err::panic_example();
    // fn_09_err::error_example();


    /* io example */
    // fn_10_io::std_out();
    // fn_10_io::cl_arg();
    // fn_10_io::file_read("inp.txt");
    // fn_10_io::file_write("out.txt","Hi There\n");
    // fn_10_io::file_append("out.txt","Hello Again\n");
    // fn_10_io::file_copy("out.txt","out2.txt");
    // fn_10_io::file_delete("out2.txt");


    /* clo */
    fn_11_clo::closure_example1(3,4);
    fn_11_clo::closure_example2(4);  
    fn_11_clo::closure_example3(5);  
    fn_11_clo::generate_force(20,3);

    /* sma */
    fn_12_sma::smaptr_example();
    fn_12_sma::smaptr_example2();

    /* con */
    // start concur_example1
    fn_13_con::concur_example();
    // end concur_example1 

    // start concur_example2
    fn_13_con::concur_example2();
    // end concur_example2
    fn_13_con::mutex_example();

    /* db */
    // app_db::crud();

    /* Uti */

    let vec = fn_14_util::split("My name is Ankita. I love Rust. I love C.");
    println!("{:?}",vec);


}



// fn f1(id: &str) {
//     let x = &id[1..3];

//     println!("{:?}", x);
// }


// #[cfg(test)] // this ensures that this is not compiled unless: cargo test
// mod my_tests {
//     #[test]
//     fn basic_test() {
//         assert_eq!(2==2, true);
//     }
// }


