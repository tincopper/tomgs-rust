/// 自定义异常处理
/// Option通过？处理
fn option_to_result() -> Option<String> {
  let str = Option::from("abc".to_string())?;
  //let r = str.ok_or("str is null")?;
  println!("{}", str);
  Option::from("a".to_string())
}

fn option_to_result1() -> Result<String, String> {
  let str = Option::from("abc".to_string()).ok_or("error".to_string())?;
  println!("str: {}", str);
  Ok("OK".to_string())
}

#[test]
fn test_option_to_result() {
  let r = option_to_result();
  println!("{}", r.unwrap());
}

#[test]
fn test_option_to_result1() {
  let r = option_to_result1();
  println!("{}", r.unwrap());
}