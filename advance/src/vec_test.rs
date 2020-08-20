
/// 定义一个动态数组，并插入数据
pub fn test_vec() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let rm = v.remove(2);
    println!("remove result: {}", rm);

    let result = v.pop();
    println!("pop result： {}", result.unwrap());

    v
}

/// 获取动态数组中的元素
pub fn test_vec_read() {
    // 方式一
    let v = vec![1, 2, 3, 4];
    let i = &v[2];
    println!("The third element is {}.", i);

    // 方式二
    let i2 = v.get(2);
    match i2 {
        Some(third) => {
            println!("The third element is {}.", third);
        },
        None => {
            println!("There is no third element.");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 实例化结构体
    #[test]
    fn test_user() {
        dbg!(test_vec());
        println!("-------------------");
        test_vec_read();
    }

}