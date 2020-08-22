// mod_org is the name of the folder; it has a file named mod.rs
mod mod_org;     
// mod_example.rs file is in "src" directory; contains mod named test
mod mod_example;  
mod io;
mod basic;

use std::process;
use mod_example::mod_test;

fn main() {
    
    /* basic concepts */
    basic::concept();
    basic::shadow();
    basic::data_types();
    basic::tuple_example();
    basic::array_example();
    basic::ctrlflow_example();
    basic::loop_example();
    process::exit(0x0100);

    /* mod example, organizing code */
    mod_org::f1();
    mod_example::mod_test::f2();

    /* io example */
    io::std_out();
    io::cl_arg();
    io::file_read("inp.txt");
    io::file_write("out.txt","Hi There\n");
    io::file_append("out.txt","Hello Again\n");
    io::file_copy("out.txt","out2.txt");
    io::file_delete("out2.txt");
}

