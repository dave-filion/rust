use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // open channels
        let (sender, recv) = mpsc::channel();

        let recv = Arc::new(Mutex::new(recv));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // create and store threads
            workers.push(Worker::new(i, Arc::clone(&recv)));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f:F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }

}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, recv: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("creating worker {}", id);
        let thread = thread::spawn(move || {
            while let Ok(job) = recv.lock().unwrap().recv() {
                println!("worker {} got a job, executing", id);
                job();
                println!("worker {} done", id);
            }
        });

        Worker{
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate
}