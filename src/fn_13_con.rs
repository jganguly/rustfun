use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};
use std::process;

pub fn concur_example() {
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("Hello # {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi # {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn concur_example2() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // clone a second producer
    let tx2 = mpsc::Sender::clone(&tx);

    // spawn a thread and move the transmitter into the closure
    // spawned thread will now own the transmitter
    thread::spawn( move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("thread-1"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // same comments of the previous code block apply here.
    thread::spawn( move || {
        let vals = vec![
            String::from("Hi"),
            String::from("your"),
            String::from("thread-2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // receive the result, timeout beyond 1 sec
    let result =
        rx.recv_timeout(Duration::from_millis(1000));

    match result { 
        Err(e) => { 
            println!("{:?}",e);
            process::exit(0);
        },
        Ok(x) => {
            for received in rx {
                println!("Got: {}", received);
            }
        }
    }
}

pub fn mutex_example() {
    let counter = Arc::new(Mutex::new(0)); // atomic reference count
    let mut handles = vec![];   // stores references to the threads

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // clone the arc

        // use the move closure and spawn 10 threads
        let handle = thread::spawn( move || {   
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // join the threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}



