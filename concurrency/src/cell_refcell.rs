use std::cell::{Cell, RefCell};
///https://www.dazhuanlan.com/2019/12/08/5deca828ede3b/
/// https://my.oschina.net/zengsai/blog/1573222
///
/// Cell 和 RefCell用于解决共享引用不可变的问题。
/// Cell 比 RefCell 更轻，性能更好，用法更方便， 但是 Cell 只能包装 Copy 类型，而 RefCell 可以包装任何类型，
/// 并且 RefCell 可以获取其内部包装对象的引用，并在运行时检测可变引用的唯一性。
/// ```
/// use std::cell::UnsafeCell;
/// struct Cell<T> {
///     value: UnsafeCell<T>,     // 内部对象
/// }
///
/// struct RefCell<T: ?Sized> {
///     borrow: Cell<usize>,      // 对象引用类别和计数
///     value: UnsafeCell<T>,     // 内部对象
/// }
/// ```
///RefCell 内部维护了一个包装对象的引用计数， 当 RefCell.borrow 获取一个共享引用时，内部引用计数加一，
/// 当获取的引用离开作用域时，内部引用计数减一， 当 RefCell.borrow_mut 获取一个可变引用时，
/// 首先检测引用技数是否为 0，如果为 0，正常返回， 如果不为 0，直接 panic，
/// 其实 RefCell.borrow 时也会做类似的检测，当已经获取了可变引用也是直接 panic，
/// 当然为了避免 panic，我们可以用 RefCell.try_borrow 和 RefCell.try_borrow_mut 来获取一个 Result 类型。
///
/// 因为 Cell 和 RefCell 两种类型都未实现 Sync trait， 所以这两种包装类型只能用于单线程中，不能跨线程操作，
/// 如果需要跨线程操作，就需要用到 Mutex 和 RwLock 了。

#[test]
fn test_borrow_checker() {
  let x = 1;
  let y = &x;
  let z = &x;
  // Error:
  //x = 2;
  //y = 3;
  //z = 4;
  println!("{}", x);
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32
}

#[test]
fn test_borrow_checker2() {
  let mut p = Point{x: 1, y: 2};
  let mut p1 = &p;
  let mut p2 = &p;
  //Error
  //p1.x = 3;
  //p2.x = 4;
  println!("{:?}", p);
}

//////////////////////////////////////////////Cell和RefCell/////////////////////////////////////////
#[test]
fn test_cell_base() {
  let x = Cell::new(1);
  let y = &x;
  let z = &x;
  x.set(2);
  y.set(3);
  z.set(4);
  println!("{:?}", x);
}

#[derive(Debug)]
struct Point1 {
  x: Cell<i32>,
  y: i32
}

#[test]
fn test_cell_base2() {
  let p = Point1{x: Cell::new(1), y: 2};
  let p1 = &p;
  let p2 = &p;
  p1.x.set(3);
  p2.x.set(4);

  println!("{:?}", p);
}

#[test]
fn test_ref_cell() {
  let x = RefCell::new(1);
  let y = &x;
  let z = &x;
  *x.borrow_mut() = 2;
  *y.borrow_mut() = 3;
  *z.borrow_mut() = 4;
  println!("{:?}", x);
}

#[derive(Debug)]
struct Point2 {
  x: RefCell<i32>,
  y: i32
}

#[test]
fn test_ref_cell2() {
  let p = Point2{x: RefCell::new(1), y: 2};
  let p1 = &p;
  let p2 = &p;
  *p1.x.borrow_mut() = 3;
  *p2.x.borrow_mut() = 4;

  println!("{:?}", p);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
struct Person {
  name: String,
  age: usize,
  count: Cell<usize>, // 一共有多少个副本，包含自身
}

impl Clone for Person {
  fn clone(&self) -> Self {
    let new_count = self.count.get() + 1;

    let p = Person {
      name: self.name.clone(),
      age: self.age,
      count: Cell::new(new_count),
    };

    self.count.set(new_count);  // ok

    p
  }
}

#[test]
fn test_cell() {
  let person = Person { name: "zengsai".to_string(), age: 18, count: Cell::new(1) };
  //    ^         ^
  //    |         |
  // 所有者     对象或值

  println!("Person count : {}", person.count.get());  // OUTPUT: Person count : 1

  let person_cloned_1: Person = person.clone();

  println!("Person count : {}", person.count.get());  // OUTPUT: Person count : 2

  let person_cloned_2: &Person = &person;

  println!("Person count : {}", person.count.get());  // OUTPUT: Person count : 3
}