use std::ops::Deref;

/// 智能指针 Box
pub fn box_test() {
  let b = Box::new(5);
  assert_eq!(*b, 5);
}

/// 解引用测试
pub fn deref_test() {
  let x = 5;
  let y = &x;
  let z = Box::new(x);

  assert_eq!(5, x);
  //assert_eq!(5, y); // error
  assert_eq!(5, *y);
  assert_eq!(5, *z); // ok
}

/// 自定义智能指针
pub struct MyBox<T>(T);

impl <T> MyBox<T> {
  fn new(t: T) -> MyBox<T> {
    MyBox(t)
  }

}

/// 实现解引用trait
impl <T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

pub fn my_box_test() {
  let a = 5;
  let b = MyBox::new(5);

  assert_eq!(a, *b);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_box() {
    println!("123");
    box_test();
  }

  #[test]
  fn test_deref() {
    deref_test();
  }

  #[test]
  fn test_my_box() {
    my_box_test();
  }

}