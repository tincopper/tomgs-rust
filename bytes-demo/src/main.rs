use bytes::{Bytes, BytesMut, Buf, BufMut};

// bytes crate demo
// bytes: A cheaply cloneable and sliceable chunk of contiguous memory.
fn main() {
  let mut buf = BytesMut::with_capacity(1024);

  // put需要传入一个Buf，其表示方法如下let mut buf: Buf = &b"hello world"[..];
  buf.put(&b"hello world"[..]);
  buf.put_u16(1234);

  let a = buf.split();
  //assert_eq!(a.bytes(), &b"hello world\x04\xD2"[..]);
  assert_eq!(a, b"hello world\x04\xD2"[..]);

  buf.put(&b"goodbye world"[..]);

  let b = buf.split();
  assert_eq!(b, b"goodbye world"[..]);

  assert_eq!(buf.capacity(), 998);
}
