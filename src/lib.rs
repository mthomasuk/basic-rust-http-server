extern crate postgres;

use postgres::{Connection, TlsMode};

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

// What even is this
trait FnBox {
    fn call_box(self: Box<Self>);
}

// No idea how this works, something about borrowing?
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// Here be dragons
type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // Prevent idiots from passing in 0 to the ThreadPool
        // size - 1 is the minimum
        let safe_size = if size < 1 { (1) } else { (size) };

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // Using with_capacity is sliiiightly more effecient
        // than just dumping an unsized vec in here
        let mut workers = Vec::with_capacity(safe_size);

        for _ in 0..safe_size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job.call_box();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

// Apparently, "Drop" runs whenever an instance of ThreadPool
// goes out of scope - that's pretty nifty
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct DB {
    pub conn: Connection,
}

impl DB {
    pub fn init(conn_string: &str) -> DB {
        let conn = Connection::connect(conn_string, TlsMode::None).unwrap();
        let db = DB { conn };
        return db;
    }
}
