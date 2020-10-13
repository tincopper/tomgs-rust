// static修饰，并且要显示指定变量类型
static HELLO_WORLD: &str = "hello world";
#[test]
fn use_global_var() {
  println!("{}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;
#[test]
fn use_mut_global_var() {
  unsafe {
    // 可变的全局变量是不安全的操作
    COUNTER += 1;
    println!("{}", COUNTER);
  }
}

const N1: i32 = 5;    //定义一个整型常量
#[test]
fn use_const_var() {
  const N: i32 = 5;    //定义一个整型常量
  println!("N1：{}， N: {}", N1, N);
}

// 声明不安全的trait
unsafe trait Foo {
  fn foo(&self);
}

// 定义一个struct
struct Bar();

unsafe impl Foo for Bar {
  fn foo(&self) {
    println!("Bar");
  }
}

#[test]
fn use_unsafe_trait() {
  let bar = Bar();
  bar.foo();
}