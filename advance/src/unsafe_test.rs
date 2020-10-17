// unsafe demo

// 裸引用测试
pub fn raw_pointer_demo() {
    let mut num = 5;
    // 可以在安全代码内合法地创建裸指针，但不能在不安全代码块外解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 直接使用报错
    // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
    //println!("r1 is {}", *r1);
    //println!("r2 is {}", *r2);

    // 使用unsafe代码块
    unsafe {
       println!("r1 is {}", *r1);
       println!("r2 is {}", *r2);
    }
}

pub fn raw_pointer_demo2() {
    // 创建一个无效引用
    // 尝试使用任意内存地址的行为是未定义的：这个地址可能有数据，也可能没有数据，
    // 编译器可能会通过优化代码来去掉该次内存访问操作，否则程序可能会在运行时出现段错误（segmentation fault）。
    let address = 0x123456usize;
    let r1 = address as *const i32;

    unsafe {
        println!("r1 is {}", *r1);
    }
}

// 不安全函数
pub unsafe fn dangerous() {
    println!("do something.");
}

// 安全抽象
pub fn unsafe_fn_abs() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// 调用c语言代码
extern "C" {
    fn abs(input: i32) -> i32;
}

// 其他语言调用rust
//#![crate_type = "staticlib"] //这个要放在文件的函数上面
#[no_mangle]
//pub extern "C" fn foo() {
pub extern fn foo() {
    print!("use rust");
}

pub fn call_c() {
    unsafe {
        let r = abs(-3);
        println!("abs(-3) is: {}", r);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_raw_pointer() {
        raw_pointer_demo();
    }

    #[test]
    pub fn test_raw_pointer2() {
        raw_pointer_demo2();
    }

    #[test]
    pub fn test_dangerous_fn() {
        unsafe {
            dangerous();
        }
    }

    #[test]
    pub fn test_call_c() {
        call_c();
    }

}
