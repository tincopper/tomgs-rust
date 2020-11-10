use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub struct ThreadPool {
  works: Vec<Worker>,
  sender: Sender<Job >,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut works = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      works.push(Worker::new(id, receiver.clone()));
    }

    ThreadPool {
      works,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    unimplemented!()
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let receiver = receiver.lock().unwrap();
        let job = receiver.recv().unwrap();
        println!("Worker {} got a job; executing.", id);
        job();
      }
    });

    Worker {
      id,
      thread,
    }
  }

}

#[test]
fn test_thread_pool() {
  let thread_pool = ThreadPool::new(5);
  for _ in 0..10 {
    thread_pool.execute(|| {
      println!("execute ...");
    });
  }

  thread::sleep(Duration::from_secs(5));
}