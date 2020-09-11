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

/// ------------------------------------------------------------------------------------------------
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

  assert_eq!(5, a);
  assert_eq!(5, *b); // *b <==> *(b.deref())
}

/// ------------------------------------------------------------------------------------------------
/// Drop trait
struct CustomSmartPointer {
  data: String
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer data `{}`.", self.data);
  }
}

#[test]
pub fn drop_test() {
  let a = CustomSmartPointer {data: String::from("a")};
  let b = CustomSmartPointer {data: String::from("b")};
  println!("CustomSmartPointer created!");
}

#[test]
pub fn drop_test2() {
  let a = CustomSmartPointer {data: String::from("a")};
  let b = CustomSmartPointer {data: String::from("b")};
  println!("CustomSmartPointer created!");
  drop(b);
  println!("CustomSmartPointer dropped before the end of main.")
}

/// ------------------------------------------------------------------------------------------------
/// 解引用自动转换
pub fn hello(name: &str) {
  println!("Hello, {}!", name);
}

pub fn ref_auto_convert_test() {
  let name = "tomgs";
  hello(name);

  let name = String::from("tomgs");
  hello(&name);

  let name = Box::new(String::from("tomgs"));
  hello(&name);

  // 如果Rust没有解引用转换功能
  // (*m)首先将MyBox<String>进行解引用得到String，然后，通过&和[..]来获取包含整个String的字符串切片以便匹配hello函数的签名。
  let name = Box::new(String::from("tomgs"));
  hello(&(*name)[..]);
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