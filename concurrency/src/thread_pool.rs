use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

enum Message {
  NewJob(Job),
  Terminate,
}

pub struct ThreadPool {
  works: Vec<Worker>,
  sender: Sender<Message>,
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
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &mut self.works {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.works {
      println!("Shutting down worker {}", worker.id);

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
  pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let receiver = receiver.lock().unwrap();
        let message = receiver.recv().unwrap();
        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job; executing.", id);
            job();
          },
          Message::Terminate => {
            println!("Worker {} was told to terminate.", id);
            break;
          }
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
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