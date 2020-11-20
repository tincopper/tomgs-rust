use futures;
use tokio::runtime::Runtime;

async fn function1() {
  println!("function1 ++++ ");
  tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await; //等待一秒钟
  function3().await;
}

async fn function2() {
  println!("function2 ++++ ");
}

async fn function3() {
  println!("function3 ++++ ");
}


async fn async_main() {
  let f1 = function1();
  let f2 = function2();

  // 使用await则会顺序执行，使用join则会并发执行f1和f2
  // f1.await;
  // f2.await;
  futures::join!(f1, f2);
}

#[test]
fn main() {
  let mut runtime = Runtime::new().unwrap();
  runtime.block_on(async_main());
}