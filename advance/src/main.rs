// 模块组织与访问权限
// 权限默认私有
mod nation {
    pub mod government {
        pub fn govern() {

        }
    }

    pub fn govern() {

    }

    mod congress {
        pub fn legislate() {

        }
    }

    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}

//如果模块中定义了结构体，结构体除了其本身是私有的以外，
//其字段也默认是私有的。所以如果想使用模块中的结构体以及其字段，需要 pub 声明
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// 枚举类枚举项可以内含字段，但不具备类似的性质:
mod SomeModule {
    pub enum Person {
        King {
            name: String
        },
        Quene
    }
}

pub fn enum_test() {
    let person = SomeModule::Person::King {
        name: String::from("Blue"),
    };
    match person {
        SomeModule::Person::King {name} => {
            println!("{}", name);
        }
        _ => {}
    }
}

//use 关键字能够将模块标识符引入当前作用域：
use crate::nation::government::govern;
use crate::nation::govern as nation_govern;
// 引入其他模块
mod second_module;
// 引入标准库
// Rust 官方标准库字典：https://doc.rust-lang.org/stable/std/all.html
use std::f64::consts::PI;

fn use_std_lib() {
    println!("{}", (PI / 2.0).sin());
}

// 错误处理
mod error_handler;
// 
mod struct_test;
mod method_test;
mod vec_test;

fn main() {
    nation::government::govern();
    // 通过use直接引用，解决了局部模块路径过长的问题
    govern();
    // 通过as 别名 解决可能同时存在两个相同的名称的问题
    nation_govern();
    eat_at_restaurant();
    enum_test();
    // 引入其他文件中的方法
    second_module::message();
    use_std_lib();
    error_handler::test_panic();
    error_handler::test_recovery();

    // struct tests
    struct_test::user_build();
    // method tests
    method_test::test_area();
    // vec tests
    vec_test::test_vec();
}
