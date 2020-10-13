mod global_var_demo;
mod arc_demo;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// mspc channel
fn mpsc_channel() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let v = String::from("hi");
    tx.send(v).unwrap();
    // send函数会获取参数的所有权，并在参数传递时将所有权转移给接收者。这可以阻止我们意外地使用已经发送的值，所有权系统会在编译时确保程序的每个部分都是符合规则的。
    // println!("{}", v); // error
  });

  let result = rx.recv().unwrap();
  println!("result: {}", result);
}

/// 发送多条消息
fn mpsc_sender_multi_msg() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi1"),
      String::from("hi2"),
      String::from("hi3")
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  // 在主线程中，我们会将rx视作迭代器，而不再显式地调用recv函数。
  for result in rx {
    //let result = rx.recv().unwrap();
    println!("result: {}", result);
  }
}

/// 使用克隆创建多个生产者
fn mpsc_multi_sender() {
  let (tx, rx) = mpsc::channel();
  // 克隆Sender
  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    let vals = vec![
      String::from("hi1"),
      String::from("hi2"),
      String::from("hi3")
    ];
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("hi4"),
      String::from("hi5"),
      String::from("hi6")
    ];
    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for result in rx {
    //let result = rx.recv().unwrap();
    println!("result: {}", result);
  }

}

#[cfg(test)]
mod tests {
  use super::*;
  use std::thread;

  #[test]
  fn test_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
      println!("vec is: {:?}", v);
    });

    //drop(v);

    handle.join().unwrap();
  }

  #[test]
  fn test_chn() {
    mpsc_channel();
  }

  #[test]
  fn test_mpsc_sender_multi_msg() {
    mpsc_sender_multi_msg();
  }

  #[test]
  fn test_mpsc_multi_sender() {
    mpsc_multi_sender();
  }

}