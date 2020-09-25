use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// rust并发编程
fn main() {
    //test_thread_create();
    test_thread_move();
}

/// 线程创建
fn test_thread_create() {

    let handler = thread::spawn(|| {
        for i in 0..10 {
            println!("thread num: {}", i);
            sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("main num: {}", i);
        sleep(Duration::from_millis(1));
    }

    // join 语句
    handler.join().unwrap();
}

/// 线程中move的使用
fn test_thread_move() {
    let v = vec![1, 2, 3, 4];
    let handler = thread::spawn(move || {
        println!("v : {:?}", v);
    });

    //drop(v);

    handler.join().unwrap();
}
