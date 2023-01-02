use std::{ 
    thread,
    sync:: { mpsc, Arc, Mutex }
};

use crate::thread_pool::*;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock().unwrap()
                .recv().unwrap();
            println!("Worker {id} got a job; executing...");
            job();
        });

        Worker { id, thread }
    }
}