use core::fmt;
use std::fmt::Formatter;

/// 高级trait测试
// 1、在trait中使用关联类型

// 2、默认泛型参数和运算符重载

// 3、父trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // 调用了to_string方法，所以需要实现Display trait
        let output = self.to_string();
        println!("outline: {}", output);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {

}

impl fmt::Display for Point {
    // 自定义输出格式
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[test]
fn test_point_out() {
    let point = Point {
        x: 1, y: 3
    };
    println!("{}", point);
    //point.outline_print();
}

//使用newtype模式在外部类型上实现外部trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test]
fn new_type_test() {
    let wrapper = Wrapper(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    println!("{}", wrapper);
}

// 新的方式
struct BetterVec<T>(Vec<T>);

impl<T> From<BetterVec<T>> for Vec<T> {
    fn from(v: BetterVec<T>) -> Self {
        // 简略转换步骤
        v.0
    }
}

#[test]
fn new_type_2() {
    let b = BetterVec::<String>(vec![String::from("a"), String::from("b")]);
    let from = Vec::from(b);
    for s in from {
        println!("{}", s);
    }
}