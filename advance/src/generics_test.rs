/// 21.3 impl块中的泛型
trait TestInto<T>: Sized {
  fn test_into(self) -> T;
}

trait TestFrom<T>: Sized {
  // 这里返回的是Self
  fn test_from(_: T) -> Self;
}

impl <T, U> TestInto<U> for T
where
    U: TestFrom<T>,
{
  fn test_into(self) -> U {
    U::test_from(self)
  }
}

impl <T> TestFrom<T> for T {
  fn test_from(t: T) -> T {
    t
  }
}

#[derive(Debug)]
struct TestString {
  name: String,
}

impl TestFrom<String> for TestString {
  fn test_from(s: String) -> TestString {
    TestString {
      name: s,
    }
  }
}

#[test]
fn test_generic() {
  let res = TestString::test_from("abc".to_string());
  println!("res: {:?}", res);

  let s: TestString = TestString::test_into(res);
  println!("s: {:?}", s);
}