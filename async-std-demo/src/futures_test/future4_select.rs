use futures::{future, select};
use tokio::runtime::Runtime;

async fn count() {
  let mut a_fut = future::ready(4);
  let mut b_fut = future::ready(6);
  let mut total = 0;

  loop {
    select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
  }
  assert_eq!(total, 10);
}

#[test]
fn main() {
  let mut runtime = Runtime::new().unwrap();
  runtime.block_on(count());
}