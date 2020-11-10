use lazy_static::*;

/// 编译器还设置了另外一条规则，即共享又可变的全局变量必须满足Sync约束。
/// 根据Sync的定义，满足这个条件的全局变量显然是线程安全的。
/// 因此，编译器把这条路也堵死了，我们不可以简单地通过全局变量共享状态来构造出线程不安全的行为。
/// 对于那些满足Sync条件且具备内部可变性的类型，比如Atomic系列类型，作为全局变量共享是完全安全且合法的。
// static修饰，并且要显示指定变量类型
static HELLO_WORLD: &str = "hello world";

// 或者使用lazy_static!宏进行定义
lazy_static! {
  #[derive(Debug)]
  static ref HELLO_WORLD1: &'static str = "hello world";
}

#[test]
fn use_global_var() {
  println!("{}", HELLO_WORLD);
  println!("{:?}", &HELLO_WORLD1.to_string());
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