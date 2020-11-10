use std::sync::{Arc, Mutex, RwLock, Barrier, Condvar};
use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::time::Duration;

/// arc、rc 修饰的变量只能共享不能进行修改
/// 记住rust中的核心：共享不可变，可变不共享
/// 在单线程环境下使用RC + Cell或RefCell的方式进行数据共享和修改
/// 在多线程环境下使用Arc + Mutex或者RwLock的方式进行数据的共享和安全的修改
#[test]
fn test_arc_1() {
  let numbers = [1, 2, 3];

  let numbers_arc = Arc::new(numbers);

  for num in 0..3 {
    //let (port, chan) = Channel::new();
    let (chan, port) = mpsc::channel();
    let _ = chan.send(numbers_arc.clone());

    // spawn要求闭包参数类型满足Send条件，闭包是没有显式impl Send或者Sync的，按照auto trait的推理规则，编译器会检查这个类型所有的成员是否满足Send或者Sync。
    thread::spawn(move || {
      let local_arc = port.recv();
      let task_numbers = &local_arc.unwrap()[..];
      println!("{:#}", task_numbers[num]);
    });
  }

}

#[test]
fn test_arc_mutex() {

  let global = Arc::new(Mutex::new(0));

  let v1 = Arc::clone(&global);
  let t1 = thread::spawn(move || {
    for _ in 0..100 {
      let mut value = v1.lock().unwrap();
      *value += 1;
    }
  });

  let v2 = Arc::clone(&global);
  let t2 = thread::spawn(move || {
    for _ in 0..100 {
      let mut value = v2.lock().unwrap();
      *value -= 1;
    }
  });

  t1.join().ok();
  t2.join().ok();

  println!("final value: {:?}", global);
}

#[test]
fn test_rw_lock() {
  let global = Arc::new(RwLock::new(0));

  let v1 = Arc::clone(&global);
  let t1 = thread::spawn(move || {
    for _ in 0..100 {
      let mut write_lock = v1.write().unwrap();
      *write_lock += 1;
    }
  });

  let v2 = Arc::clone(&global);
  let t2 = thread::spawn(move || {
    for _ in 0..100 {
      let mut write_lock = v2.write().unwrap();
      *write_lock -= 1;
    }

  });

  t1.join().ok();
  t2.join().ok();

  println!("final value: {:?}", global);
}

/// Rust标准库还为我们提供了一系列的“原子操作”数据类型，它们在std::sync::atomic模块里面。它们都是符合Sync的，可以在多线程之间共享。
#[test]
fn test_atomic() {
  let global = Arc::new(AtomicIsize::new(0));

  let v1 = Arc::clone(&global);
  let t1 = thread::spawn(move || {
    for _ in 0..100 {
      v1.fetch_add(1, Ordering::SeqCst);
    }
  });

  let v2 = Arc::clone(&global);
  let t2 = thread::spawn(move || {
    for _ in 0..100 {
      v2.fetch_sub(1, Ordering::SeqCst);
    }
  });

  t1.join().ok();
  t2.join().ok();

  println!("final value: {:?}", global);
}

/// Barrier是这样的一个类型，它使用一个整数做初始化，可以使得多个线程在某个点上一起等待，然后再继续执行。
#[test]
fn test_barrier() {
  let barrier = Arc::new(Barrier::new(10));
  let mut join_handles = vec![];

  for _ in 0..10 {
    let c = barrier.clone();

    let t = thread::spawn(move || {
      println!("before wait.");
      c.wait();
      println!("after wait.");
    });

    join_handles.push(t);
  }

  for handler in join_handles {
    handler.join().ok();
  }
}

/// Condvar是条件变量，它可以用于等待某个事件的发生。在等待的时候，这个线程处于阻塞状态，并不消耗CPU资源。
/// 在常见的操作系统上，Condvar的内部实现是调用的操作系统提供的条件变量。
/// 它调用wait方法的时候需要一个MutexGuard类型的参数，因此Condvar总是与Mutex配合使用的。
#[test]
fn test_condvar() {
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = pair.clone();

  thread::spawn(move || {
    thread::sleep(Duration::from_secs(3));
    let &(ref lock, ref cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();

    println!("child thread started: {}", *started);
  });

  let &(ref lock, ref cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  println!("befor wait: {}", *started);
  while !*started {
    started = cvar.wait(started).unwrap();
  }
  println!("after wait: {}", *started);

  //thread::sleep(Duration::from_secs(3));
}