
/// 定义一个动态数组，并插入数据
pub fn test_vec() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    v.insert(1, 4);
    v.insert(1, 5);

    for i in 0..v.len() {
        println!("a[{}] = {}", i, v[i]);
    }

    let rm = v.remove(2);
    println!("remove result: {}", rm);

    let result = v.pop();
    println!("pop result： {}", result.unwrap());

    //let result = (0u32..20).map(|f| f + 1).collect();
    //println!("{}", result);

    let result = (0u32..20).map(|f| f + 1);
    println!("{:?}", result);

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

/// 使用iter遍历vec
pub fn vec_iter_test() {
    let v = vec![1, 2, 3, 4];
    // 这一步不会对vec有什么操作
    let v_iter = v.iter();

    for e in v_iter {
        println!("Got {}.", e)
    }

}

pub fn vec_iter_test2() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
    assert_eq!(v_iter.next(), None);
}

pub fn vec_iter_sum_test() {
    let v = vec![1, 2, 3];
    let total: i32 = v.iter().sum();

    assert_eq!(total, 6);
}

pub fn vec_iter_map_test() {
    let v = vec![1, 2, 3];
    let result: Vec<_> = v.iter().map(|x| x + 1).collect();
    assert_eq!(result, vec![2, 3, 4]);
}

/// 使用filter
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let result: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == shoe_size).collect();
    result
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

    #[test]
    fn test_vec_iter() {
        vec_iter_test();
        vec_iter_test2();
        vec_iter_sum_test();
        vec_iter_map_test();
    }

    #[test]
    fn test_filters_by_size() {
        let shoes = vec![
            Shoe{size: 10, style: String::from("sneaker")},
            Shoe{size: 13, style: String::from("sandal")},
            Shoe{size: 10, style: String::from("boot")},
        ];
        let result = shoes_in_my_size(shoes, 10);
        assert_eq!(result, vec![
            Shoe{size: 10, style: String::from("sneaker")},
            Shoe{size: 10, style: String::from("boot")},
        ])
    }

}