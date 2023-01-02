use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Simple thread pool implementation using job queue.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new thread pool with the given number of workers.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // Create mpsc channel
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // Create the workers
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Sends the closure for execution by the thread pool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(threat) = worker.thread.take() {
                threat.join().unwrap();
            }
        }
    }
}

/// Represents a single worker thread in the thread pool.
struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new worker that checks for jobs from the given receiver channel half.
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => job(),
                Err(_) => break,
            }
        });
        Worker {
            thread: Some(thread),
        }
    }
}
