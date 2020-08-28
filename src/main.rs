// mod_org is the name of the folder; it has a file named mod.rs
mod mod_org;     
// mod_example.rs file is in "src" directory; contains mod named test
mod mod_example;  
mod io;
mod basic;
mod own;
mod st;
mod tra;
mod enu;
mod util;


use mod_example::mod_test;


fn main() {
    
    /* basic concepts */

    // basic::concept();
    // basic::shadow();
    // basic::data_types();
    // basic::tuple_example();
    // basic::array_example();
    // basic::ctrlflow_example();
    // basic::loop_example();


    /* mod example, organizing code */

    // mod_org::f1();
    // mod_example::mod_test::f2();

    /* io example */
    // io::std_out();
    // io::cl_arg();
    // io::file_read("inp.txt");
    // io::file_write("out.txt","Hi There\n");
    // io::file_append("out.txt","Hello Again\n");
    // io::file_copy("out.txt","out2.txt");
    // io::file_delete("out2.txt");


    /* own */

    // own::example1();
    // own::example2();
    // own::example3();

    // let mut x: i32 = 42;
    // own::example5(&mut x);  // x is passed by reference
    // println!("{}\n",x);

    // let mut x: i32 = 42;
    // let y = own::example6(&mut x);
    // println!("Returned value {:?}\n",y);

    // let mut s = "Hello";
    // let s = own::example7(&mut s);
    // println!("Returned slice {:?}\n",s);


    // let x = own::create_series(10);
    // println!("series: {:?}",x); 

    // own::copy_trait_example();
    // own::struct_copy_example();
    // own::ref_example();
    // own::mut_ref_example();
    // own::mut_ref_restrict();
    // own::mut_ref_restrict2();
    // own::mut_ref_example2(); 

    // let alice = "Alice";
    // let bob = "Bob";
    // println!("{}", own::lifetime_example(alice, bob));

    /* st */

    // let f = st::Function{};
    // f.say_hello();

    // let r = st::Rect {id:"abc",width:10, length:20};
    // println!("Area = {}",r.area());
    // println!("Volume = {}",r.volume(10));

    // let mut person = 
    //     st::Person { 
    //         name: "Tim",
    //         id: 0, 
    //         amt: 52.54
    //     };
    // let id = 900;
    // person.set_id(&id);
    // println!("{:?}", person);

    // let mut x = String::from("Jaideep");
    // f1(&x);


    /* tra */
    // use tra::Animal;
    // let h = tra::Herbivore;
    // h.eat();

    // let c = tra::Carnivore;
    // c.eat();

    // use tra::Activity;
    // let eagle = tra::Eagle;
    // eagle.fly();
    // tra::activity(eagle);

    // let hen = tra::Hen;
    // tra::activity(hen);      // Error

    // let sound_book = tra::SoundBook {
    //     sounds: vec! [
    //         Box::new(tra::Horse{}),
    //         Box::new(tra::Deer{}),
    //         Box::new(tra::Tiger{}),
    //         Box::new(tra::Duck{})
    //     ]
    // };

    // sound_book.run();

    /* enu */
    use enu::Gender;
    let male = enu::Gender::Male;
    let female = enu::Gender::Female;
    println!("{:?}",male);
    println!("{:?}",female);
    

    let my_black = enu::MyBlack {
        name: String::from("my black"), 
        rgb: (10,10,10)
    };
    let black = enu::Color::Black(my_black);
    let white = enu::Color::White(255,255,255);
    println!("{:?}",black);
    println!("{:?}",white);
    white.printColor();

    let s = Some(803);
    assert_eq! (s.is_some(), true);
    println!("{}",s.unwrap());

    match black {
        enu::Color::White(x,y,z) => println!("{} {} {}",x,y,z),
        enu::Color::Black(x) => println!("{:?}",x.rgb),
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
    
    enu::match_example(4);

    /* Util */

    // util::handy();
    // let vec = util::split("My name is Ankita. I love Rust. I love C.");
    // println!("{:?}",vec);

    // util::vec_example();

}

//  fn example1() {

    
//     pub trait Animal {
//          fn eat(&self);
//     }

//     pub struct Herbivore;
//     struct Carnivore;

//     impl Animal for Herbivore {
//          fn eat(&self) {
//             println!("I eat plants");
//         }
//     }

//     impl Animal for Carnivore {
//         fn eat(&self) {
//             println!("I eat meat");
//         }
//     }

// }

fn f1(id: &str) {
    let x = &id[1..3];

    println!("{:?}", x);
}

