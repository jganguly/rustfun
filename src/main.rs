// mod_org is the name of the folder; it has a file named mod.rs
mod mod_org;     
// mod_example.rs file is in "src" directory; contains mod named test
mod mod_example;  
mod io;
mod basic;
mod own;
mod st;
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

    let mut x: i32 = 42;
    own::example5(&mut x);  // x is passed by reference
    println!("{}\n",x);

    let mut x: i32 = 42;
    let y = own::example6(&mut x);
    println!("Returned value {:?}\n",y);

    let mut s = "Hello";
    let s = own::example7(&mut s);
    println!("Returned slice {:?}\n",s);


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
    // let r = st::Rect {w:10, h:20};
    // println!("{}",r.area());

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


    // let f = st::Function{};
    // f.say_hello();


    /* Util */

    // util::handy();
    // let vec = util::split("My name is Ankita. I love Rust. I love C.");
    // println!("{:?}",vec);

    // util::vec_example();







}

fn f1(id: &str) {
    let x = &id[1..3];

    println!("{:?}", x);
}

