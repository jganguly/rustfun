use std::sync::{Arc,mpsc,Mutex};
use std::thread;

// struct ThreadPool
pub struct ThreadPool {
    workers: Vec<Worker>, // vector of worker thread
    sender: mpsc::Sender<Message>,
}

// Define the type for Job, note it is dyn,
// implements traits FnOnce & Send and lifetime is 'static
type Job = Box<dyn FnOnce() + Send + 'static>;

// enum
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {

pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // 'new' function will panic if size is 0
    let (sender, receiver) = mpsc::channel();

    // Use Arc<Mutex<T>> so that thread-safe smart pointers share
    // ownership across multiple threads and allow the threads
    // to mutate the value. The Arc type will let multiple
    // workers own the receiver, and Mutex will ensure that only
    // one worker gets a job from the receiver at a time.
    // In ThreadPool::new,the receiving end of the channel is put
    // in an Arc (Atomically Reference Counted) and a Mutex.
    let receiver = Arc::new(Mutex::new(receiver));

    // Creat the vector
    let mut workers = Vec::with_capacity(size);

    // For each new worker, clone the Arc to bump the reference
    // count so workers can share ownership of the receiving end.
    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    // Expresson to be returned
    ThreadPool { workers, sender }
}

// F type parameter has trait bound Send & lifetime bound 'static
// We use the () after FnOnce because FnOnce represents a
// closure that takes no parameters and returns the unit type ()
// Note: As in functions, the return type can be omitted from the
// signature, but even if we have no parameters,
// we still need the parentheses.
// 'Send' is needed to transfer the closure from one thread to
// another and 'static because we do not know how long the thread
// will take to execute.
pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {

    // Create the job instance
    let job = Box::new(f);
    // Send the job
    self.sender.send(Message::NewJob(job)).unwrap();
    }
}



 // Implement Drop trait for ThreadPool
impl Drop for ThreadPool {

    fn drop(&mut self) {

        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// struct Worker
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        // thread::spawn will create a new thread and run the
        // code in the closure in the new thread.
        // Closure to the loop, ask the receiving end of the channel
        // for a job and run the job when it gets one.
        let thread = thread::spawn( move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }          
        });

        // Expression returned
        Worker {
            id,
            thread: Some(thread),
        }
    }
}