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

}