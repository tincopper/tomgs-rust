//函数指针与闭包

fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

#[test]
fn test_do_twice() {
  // 传入一个函数
  let res = do_twice(add_one, 1);
  println!("result: {}", res);
}

#[test]
fn test_do_twice1() {
  // 传入一个闭包
  let x = 2;
  let res = do_twice(|x| x + 1 , 1);
  println!("result: {}", res);
}

///////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Status {
  Value(u32),
  Stop,
}

#[test]
fn list_of_status() {
  let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
  println!("{:?}", list_of_status);
}

///////////////////////////////////////////////////////////////////////////
// error
//fn return_closure() -> Fn(i32) -> i32 {
//  |x| x + 1
//}

// correct
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

#[test]
fn test_return_closure() {
  let res = (*return_closure())(1);
  println!("{}", res);
}

/// nom:
/// ```
/// pub fn tuple<I: Clone, O, E: ParseError<I>, List: Tuple<I,O,E>>(l: List)  -> impl Fn(I) -> IResult<I, O, E> {
///  move |i: I| {
///    l.parse(i)
///  }
///}
/// ```
pub fn return_closure2() -> impl Fn(i32) -> i32 {
  |x| x + 1
}

#[test]
fn test_return_closure1() {
  let res = return_closure2()(1);
  println!("{}", res);
}