
// 不带参数的函数
fn function() {
    println!("this is new function!");
}

// 带参数的函数
fn function1(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

// 函数体表达式与嵌套函数
fn function2() {
    let x = 5;
    // 函数体表达式的使用，表达式的结果值是整个表达式块所代表的值
    let y = {
        let x = 3;
        x + 1 // 不需要分号，带分号表示是一个语句
    };

    println!("x值：{}", x);
    println!("y值：{}", y);

    // 嵌套函数
    fn five() -> i32 {
        5
    }
    println!("five()的值为:{}", five());
}

// 变量的重影
fn function3() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("x值：{}", x);
}
// 函数的返回值
// 注意：函数体表达式并不能等同于函数体，它不能使用 return 关键字。
fn function4() {
    let result = add(1, 2);
    println!("1 + 2 = {}", result);
}

// 使用 -> 符合来表示函数的返回值
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// 条件表达式
fn function5() {
    // 和传统的条件表达式一样的用法，但是条件不需要括号了，但是也是支持括号的
    // 条件必须为布尔类型表达式，不能使用1或者0来表示真假
    let number = 5;
    if number < 6 {
        println!("true");
    } else {
        println!("false");
    }

    // 模拟3元表达式
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}

// 循环
fn function6() {
    // while用来代替for (i = 0; i < 10; i++)，而且不支持do while
    let mut number = 1;
    while number != 4 {
        println!("number is {}", number);
        number += 1;
    }
    println!("EXIT");

    // for 循环是最常用的循环结构，常用来遍历一个线性数据据结构（比如数组）。for 循环遍历数组：
    let arr = [10, 20, 30, 40, 50];
    for i in arr.iter() {
        println!("值为：{}", i);
    }

    // for 循环其实是可以通过下标来访问数组的
    for i in 0..5 {
        println!("a[{}] = {}", i, arr[i]);
    }

    // loop 循环，无限循环
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}

// 移动变量
// 移动变量不会消耗很多系统时间，除去基本数据类型会进行数据复制，引用类型只是将引用执行数据所在堆的位置
fn function7() {
    let s1 = String::from("hello");
    let s2 = s1; // 进行移动后的变量将会失效
    //println!("{}, world!", s1); // 错误！s1 已经失效
}

// 克隆变量
// 克隆仅在需要复制的情况下使用，毕竟复制数据会花费更多的时间。
fn function8() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆变量之后变量不会失效
    println!("s1 = {}, s2 = {}", s1, s2); // s1变量还是可以进行使用
}

// 字符串切片
fn function9() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);
}

fn function10() {
    let mut s = String::from("runoob");
    let slice = &s[0..3];
    //s.push_str("yes!"); // 错误
    //println!("slice = {}", slice);
}

#[derive(Debug)]
enum Book {
    Papery, Electronic
}
// 枚举类型
fn function11() {
    let book = Book::Papery;
    println!("{:?}", book);
}

// 空值处理与match语法
fn function12() {
    // 正常写法
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 简化写法
    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    // if let语法
    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {},
    }

    // 简化if let
    /*
        if let 匹配值 = 源变量 {
            语句块
        }
    */
    if let 0 = i {
        println!("zero");
    }
}

fn function13() {
    enum Book {
        Papery(u32),
        Electronic(String)
    }

    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}

fn function14() {
    let a = [1, 2, 3, 4, 5];
    // 数组越界
    // let index = 10;
    let index = 2;
    let e = a[index];
    println!("The value is: {}", e);
}

fn main() {
    println!("Hello, world!");
    function();
    function1(12, 23);
    function2();
    function3();
    function4();
    function5();
    function6();
    //function7();
    function8();
    function9();
    function10();
    function11();
    function12();
    function13();
    function14();
}
