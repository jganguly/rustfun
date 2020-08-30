use rust_svr::ThreadPool;
use std::fs; // file system I/O
use std::io::prelude::*; // stream I/O
use std::net::{TcpListener, TcpStream};

fn main() {

    // The bind function works like the new function, 
    // returns a new TcpListener instance. The bind function 
    // returns Result<T,E>, it will unwrap & stop on error.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // If we created a new thread for each request as it came in, 
    // someone making 10 million requests to our server could use up 
    // server resources. Limit the number of threads in the pool to a 
    // small number to protect us from DoS attacks. Each of the 
    // threads in the pool will pop off a long running request from 
    // this queue, handle the request, and then ask the queue for 
    // another request. Subsequent requests will back up in the 
    // queue. We will process 4 requests. 
    let pool = ThreadPool::new(4);
    
    // The incoming method on TcpListener returns an iterator that 
    // gives us a sequence of TcpStreams. A single stream represents 
    // an open connection between the client & server. The TcpStream 
    // will read from itself to see what the client sent and then 
    // allow us to write the response to the stream. This 'for' loop 
    // will process each connection in turn and produce a series of 
    // streams for us to handle.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {   
    // mut on stream is required because it might read more data than
    // asked for and save that data for the next time we ask for data.
    let mut buffer = [0; 1024];         // big enough for now
    stream.read(&mut buffer).unwrap();  // read from stream into buffer

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html") // file at ../src
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
