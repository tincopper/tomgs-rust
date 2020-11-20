/*
select 中使用的 Future 必须实现 Unpin trait 和 FusedFuture trait。

必须实现 unpin 的原因是，select 中使用的 future 不是按值获取的，而是按照可变引用获取的，通过不获取 future 的所有权，所以在调用 select 后，未完成的 future 可以继续使用。

必须实现 FusedFuture 的原因，select 完成后不会再轮询 future，因此需要实现 FusedFuture 来跟踪 future 是否完成。

同样的，对应到 stream 上，会有一个 FusedStream trait。

其他：
Fuse::terminated () 允许构建一个已经终止的空的 Future；
当需要同时运行同一 Future 的多个副本时，可以使用 FuturesUnordered 类型。
 */
use futures::{
  stream::{Stream, StreamExt, FusedStream},
  select,
};
use tokio::runtime::Runtime;

async fn add_two_streams(
  mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
  mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
  let mut total = 0;

  loop {
    let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
    if let Some(next_num) = item {
      total += next_num;
    }
  }

  total
}

// #[test]
// fn main() {
//   let mut runtime = Runtime::new().unwrap();
//   runtime.block_on(add_two_streams());
// }