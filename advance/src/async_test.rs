
async fn say_hi() {
    println!("nice");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    // 实例化结构体
    #[test]
    fn test_async() {
        let future = say_hi();
        // block the current thread until provided future
        thread::sleep(Duration::from_secs(5));
        println!("hello");
    }

}
