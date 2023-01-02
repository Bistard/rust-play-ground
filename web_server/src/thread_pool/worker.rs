use std::{ 
    thread,
    sync:: { mpsc, Arc, Mutex }
};

use crate::thread_pool::*;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
                
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing...");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down...");
                    break;
                }
            }
        });

        Worker { 
            id, 
            thread: Some(thread),
        }
    }
}