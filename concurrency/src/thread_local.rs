use std::cell::RefCell;
use std::thread;

/// 线程局部（Thread Local）的意思是，声明的这个变量看起来是一个变量，
/// 但它实际上在每一个线程中分别有自己独立的存储地址，是不同的变量，互不干扰。
/// 在不同线程中，只能看到与当前线程相关联的那个副本，因此对它的读写无须考虑线程安全问题。
///
///在Rust中，线程独立存储有两种使用方式。
/// ❏ 可以使用#[thread_local] attribute来实现。这个功能目前在稳定版中还不支持，只能在nightly版本中开启#! [feature(thread_local)]功能才能使用。
/// ❏ 可以使用thread_local！宏来实现。这个功能已经在稳定版中获得支持。

#[test]
fn test_thread_local() {
  thread_local! {
    pub static FOO: RefCell<u32> = RefCell::new(1)
  };

  FOO.with(|f| {
    println!("main thread value1: {:?}", *f.borrow());
    *f.borrow_mut() = 2;
    println!("main thread value2: {:?}", *f.borrow());
  });

  let t = thread::spawn(move || {
    FOO.with(|f| {
      println!("child thread value1: {:?}", *f.borrow());
      *f.borrow_mut() = 3;
      println!("child thread value2: {:?}", *f.borrow());
    });
  });
  t.join().ok();

  FOO.with(|f| {
    println!("main thread value1: {:?}", *f.borrow());
  });

}