// select 宏也允许并发的执行 Future，但是和 join、try_join 不同的是，select 宏只要有一个 Future 返回，就会返回。
use futures::{select, future::FutureExt, pin_mut};
use tokio::runtime::Runtime;
use std::io::Result;

async fn function1() -> Result<()> {
  tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
  println!("function1 ++++ ");
  Ok(())
}

async fn function2() -> Result<()> {
  println!("function2 ++++ ");
  Ok(())
}

async fn async_main() {
  let f1 = function1().fuse();
  let f2 = function2().fuse();

  pin_mut!(f1, f2);

  select! {
        _ = f1 => println!("task one completed first"),
        _ = f2 => println!("task two completed first"),
    }
}

#[test]
fn main() {
  let mut runtime = Runtime::new().unwrap();
  runtime.block_on(async_main());
}