// rust错误处理
/*
Rust 有一套独特的处理异常情况的机制，它并不像其它语言中的 try 机制那样简单。
首先，程序中一般会出现两种错误：可恢复错误和不可恢复错误。
可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。
但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。
大多数编程语言不区分这两种错误，并用 Exception （异常）类来表示错误。在 Rust 中没有 Exception。
对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。
*/
use std::io;
use std::io::{Read, ErrorKind, Error};
use std::fs::File;

pub fn test_panic() {
    //panic!("error occured");
    println!("Hello, Rust");
}

// 可恢复的错误
pub fn test_recovery() {
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file");
    }
}

pub fn test_recovery2() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("There was problem opening the file {:?}", e),
    };
}

pub fn test_recovery3() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("There was problem opening the file {:?}", e),
    };
}

pub fn test_recovery4() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("create file error: {:?}", e),
            },
            other_err=> panic!("opening file error: {:?}", other_err),
        }
    };
}

pub fn test_recovery5() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("create file error: {:?}", e);
            })
        } else {
            panic!("opening file error: {:?}", other_err)
        }
    });
}

/// 验证错误传播
pub fn read_username_from_file() -> Result<String, Error> {
    let file = File::open("hello.txt");
    let mut file = match file {
        Ok(f) => f,
        // 出现异常直接返回异常
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    // 返回一个Result
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file1() -> Result<String, Error> {
    let mut file = File::open("hello.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file2() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("hello.txt")?;
    Ok(())
}

pub fn test_inner_error_handle() {
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open");
}

//可恢复错误的传递
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

pub fn test_error_transport() {
    let r = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}
// 函数 g 传递了函数 f 可能出现的错误（这里的 g 只是一个简单的例子，实际上传递错误的函数一般还包含很多其它操作）。
fn g(i: i32) -> Result<i32, bool> {
    let t = f(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}

// Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去：
// ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
// 所以，? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致。
fn g_1(i: i32) -> Result<i32, bool> {
    let t = f(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

pub fn test_error_transport2() {
    let r = g_1(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}

// Kind方法
fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn test_error_kind() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}