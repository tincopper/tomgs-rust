/**
 * 模式匹配学习
 * 
 * 模式有多种形式：字面量面试、标识符模式、通配符模式、剩余模式、区间模式、引用模式、结构体模式、元组结构体模式、
 * 元组模式、分组模式、切片模式、路径模式、或模式。
 * 通俗来讲，通过模式匹配，判断表达式的值和模式是否相等、是否存在、是否在某个范围内、是否为真等。
 * 
 * 同 Go 的 switch 一样，Rust 的 match 也是不能向下穿透的，但 Rust 中不存在类似 Go 中的 fallthrough 来强制向下穿透。实际上，Rust 也不提倡各个分支的模式相互重合。
 * 另外，Rust 的 match 与 Go 的 switch 还存在一个重要差别，就是 match 表达式的匹配必须是穷尽的。
 */
#[cfg(test)]
mod tests {

    /**
     * 其中 0 模式只有一个字面量构成。
     * 1 | 2 和 3 | 4 | 5 属于或模式，它使用 | 将多个由字面量构成的模式组合起来，x 的值若匹配模式中的任何一个值，该分支将会被执行。
     * 最后一个 _ 属于通配符模式，_ 能与任何值匹配，因此相当于 Go 的 switch 中的 default 分支，但该分支必须放置在所有分支的最后。
     */
    #[test]
    fn test_match_mod() {
        let x: u32 = 3;
        match x {
            0 => println!("x = 0"),
            1 | 2 => println!("1 <= x <= 2"),
            3 | 4 | 5 => println!("3 <= x <= 5"),
            _ => println!("x > 5"),
        }
    }

    /**
     * 这里 t 是一个简单的标识符模式，由于检验对象 x 和模式 t 匹配，x 的值被赋值给模式中的变量 t 了。
     */
    #[test]
    fn test_match_mod1() {
        let x: u32 = 6;
        match x {
            0 => println!("x = 0"),
            1 | 2 => println!("1 <= x <= 2"),
            3 | 4 | 5 => println!("3 <= x <= 5"),
            t => println!("x = {}, x > 5", t),
        }
    }

    /**
     * 代码中的 x 同样属于标识符模式，一旦匹配，hour 的值将赋值给 x。但 x 的后面还跟着 if (1.0..5.0).contains(&x) 形式的内容，这叫作匹配守卫（match guard）。
     * 匹配守卫是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
     */
    #[test]
    fn test_match_mod2() {
        let result = time_period_name(4.0_f64);
        println!("result = {}", result)
    }

    pub fn time_period_name(hour: f64) -> String {
        match hour {
            x if (1.0..5.0).contains(&x) => String::from("凌晨"),
            x if (5.0..8.0).contains(&x) => String::from("早上"),
            x if (8.0..11.0).contains(&x) => String::from("上午"),
            x if (11.0..13.0).contains(&x) => String::from("中午"),
            x if (13.0..17.0).contains(&x) => String::from("下午"),
            x if (17.0..19.0).contains(&x) => String::from("傍晚"),
            x if (19.0..23.0).contains(&x) => String::from("晚上"),
            x if (23.0..24.0).contains(&x) || (0.0..1.0).contains(&x) => String::from("晚上"),
            _ => String::from(""),
        }
    }

    #[test]
    fn test_match_mod3() {
        let x = Some(3);
        match x {
            Some(0) => println!("x = 0"),
            Some(1) | Some(2) => println!("1 <= x <= 2"),
            Some(3) | Some(4) | Some(5) => println!("3 <= x <= 5"),
            Some(t) => println!("x = {}, x > 5", t),
            None => println!("null value"),
        }
    }

    /**
     * 当只需要匹配一个模式而忽略其他模式时，可以用 if let 表达式代替 match 表达式
     */
    #[test]
    fn test_match_mod4() {
        let score: u32 = 82;
        match score {
            60..=100 => println!("及格"),
            _ => println!("不及格"),
        }

        let score: u32 = 82;
        if let 60..=100 = score {
            println!("及格");
        } else {
            println!("不及格");
        }
    }
}