//src/main.rs
use futures::try_join;
use tokio::runtime::Runtime;
use std::io::Result;

async fn function1() -> Result<()> {
  tokio::time::delay_for(tokio::time::Duration::from_secs(10)).await;
  println!("function1 ++++ ");
  Ok(())
}

async fn function2() -> Result<()> {
  println!("function2 ++++ ");
  Ok(())
}

async fn async_main() {
  let f1 = function1();
  let f2 = function2();

  // f1.await;
  // f2.await;
  if let Err(_) = try_join!(f1, f2) {
    println!("Err!")
  }
}

#[test]
fn main() {
  let mut runtime = Runtime::new().unwrap();
  runtime.block_on(async_main());
  println!("Hello, world!");
}