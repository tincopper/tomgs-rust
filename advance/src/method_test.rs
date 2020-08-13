
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 实现Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 带参数的
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数：因为这类函数与结构体相互关联
    // 将其命名为函数而不是方法，是因为它们不会作用于某个具体的结构体实例
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

pub fn test_area() {
    let rectangle = Rectangle {width: 4, height: 2};
    // 方法直接用.号就可以进行调用
    let size = rectangle.area();
    println!("rectangle size is : {}", size);

    let rectangle1 = Rectangle {width: 2, height: 1};
    let is_hold = rectangle.can_hold(&rectangle1);
    println!("can hold : {}", is_hold);

    // 这个就类似于 String::from("test")
    let r = Rectangle::square(20);
    println!("square info: {:#?}", r);
}