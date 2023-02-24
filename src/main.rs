use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;


fn main() {
    vase_problem(50, 100);
}

// n - number of threads
// fn birthday_problem(n: usize) {
    

// }

// n - number of threads
fn vase_problem(n: usize, iterations: usize) {
    
    // sign simulates the sign on the door
    let sign = Arc::new(Mutex::new(true));

    // initialize the thread pool (guests)
    let pool = ThreadPool::new(n);

    // push the jobs to the pool
    for i in 0..iterations {
        println!("Iteration {} starting...", i);

        let c_sign = sign.clone();

        pool.execute(move || {
            // acquire lock
            let mut c_sign = c_sign.lock().unwrap();

            // flip the sign
            *c_sign = false; 
            
            // look at the vase
            thread::sleep(Duration::from_secs(1));

            // flip the sign back
            *c_sign = true;
        })
    }

    // drop the pool (dismiss the guests)
    drop(pool);

    println!("Jobs Done!");

}


// thread pool stuff
// mostly from rust book multithreaded server tutorial I did ages ago
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

// these run generic closures
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// defines a closure
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {

    // constructor
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // returns 
        ThreadPool {
            workers,
            sender,
        }
    }

    // execute method takes in a function and sends it to a thread
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// needed to drop the threads 
enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // recieve the message from the sender
                let message = reciever.lock().unwrap().recv().unwrap();

                // if new job do the job
                // if terminate, stop waiting for jobs
                match message {
                    Message::NewJob(job) => {
                        job.call_box();

                        println!("Guest {} viewing vase", id);
                    },
                    Message::Terminate => {
                        break;
                    },
                }
            }
        });

        Worker { 
            id, 
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    // close all threads
    fn drop(&mut self) {
        for _ in &mut self.workers {
            
            // send the termintion message to all threads
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("All guest have left!");
    }
}