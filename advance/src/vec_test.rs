
// 定义一个动态数组，并插入数据
pub fn test_vec() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // 实例化结构体
    #[test]
    fn test_user() {
        println!("123123131");
        dbg!(test_vec());
    }

}