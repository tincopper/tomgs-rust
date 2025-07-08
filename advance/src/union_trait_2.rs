// 关联类型示例
// 判断容器是否包含两个元素
trait Contains<A, B> {
    fn contains(&self, a :&A, b :&B) -> bool;
    //fn first(&self) -> A;
    fn first(&self) -> i32;
    
    //fn last(&self) -> B;
    fn last(&self) -> i32;
}

struct Container(i32, i32);

impl Contains<i32, i32> for Container {
    fn contains(&self, a :&i32, b :&i32) -> bool {
        &self.0 == a && &self.1 == b
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦。
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}


#[test]
fn test_union_trait() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}