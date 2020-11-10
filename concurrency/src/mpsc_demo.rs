use std::sync::mpsc;
use std::thread;
use std::thread::spawn;

/// 一种线程之间的通信方式：mpsc。这部分的库存储在std::sync::mpsc这个模块中。
/// mpsc代表的是Multi-producer,single-consumer FIFO queue，即多生产者单消费者先进先出队列。
/// 这种线程之间的通信方式是在不同线程之间建立一个通信“管道”（channel），一边发送消息，一边接收消息，完成信息交流。
/// 异步管道：
/// 异步管道内部有一个不限长度的缓冲区，可以一直往里面填充数据，直至内存资源耗尽。
/// 异步管道的发送端调用send方法不会发生阻塞，只要把消息加入到缓冲区，它就马上返回。
#[test]
fn test_channel() {
  let (sender, receiver) = mpsc::channel();
  thread::spawn(move || {
    for i in 0..10 {
      sender.send(i).unwrap();
    }
  });

  while let Ok(r) = receiver.recv() {
    println!("r: {}", r);
  }
}

/// 异步管道2：
#[test]
fn test_channel2() {
  let (sender, receiver) = mpsc::channel();
  for i in 0..10 {
    let sender = sender.clone();
    thread::spawn(move || {
      sender.send(i).unwrap();
    });
  }

  drop(sender);

  while let Ok(r) = receiver.recv() {
    println!("r: {}", r);
  }
}

/// 同步管道
/// 同步管道的特点是：其内部有一个固定大小的缓冲区，用来缓存消息。
/// 如果缓冲区被填满了，继续调用send方法的时候会发生阻塞，等待接收端把缓冲区内的消息拿走才能继续发送。
/// 缓冲区的长度可以在建立管道的时候设置，而且0是有效数值。
/// 如果缓冲区的长度设置为0，那就意味着每次的发送操作都会进入等待状态，直到这个消息被接收端取走才能返回。
#[test]
fn test_sync_channel() {
  let (tx, rx) = mpsc::sync_channel(1);
  tx.send(1).unwrap();
  println!("send first.");

  thread::spawn(move || {
    tx.send(2).unwrap();
    println!("send second.");
  });

  println!("receiver first : {}", rx.recv().unwrap());
  println!("receiver second : {}", rx.recv().unwrap());
}