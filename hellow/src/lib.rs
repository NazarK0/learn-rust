use std::thread;
use std::sync::{mpsc, Arc, Mutex};

struct Worker {
  id: usize,
  item: thread::JoinHandle<()>,
}

impl Worker {
  fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let item = thread::spawn(move || loop {
      let job = receiver
        .lock()
        .unwrap()
        .recv()
        .unwrap();
      
      println!("Worker {} got a job; executing.", id);
      
      job();
    });
    Worker { id, item }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  /// Creates a new thread pool
  /// 
  /// The size is the number of threads in the pool.
  /// 
  /// #Panics
  /// 
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}
