use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
/*
fn main() {
  let numbers = [1, 2, 3];

  let numbers_arc = Arc::new(numbers);

  for num in range(0, 3) {
    //let (port, chan) = Channel::new();
    let (chan, port) = mpsc::channel();
    chan.send(numbers_arc.clone());

    thread::spawn({
      let local_arc = port.recv();
      let task_numbers = local_arc.get();
      println!("{:#}", task_numbers[num]);
    });
  }

}*/