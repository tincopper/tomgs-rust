
async fn say_hi() {
    println!("nice");
}

#[async_std::test]
async fn main() -> Result<(), surf::Error> {
    // femme::start(log::LevelFilter::Info)?;

    let uri = "http://httpbin.org/post";
    // let uri = "https://httpbin.org/post";
    let data = serde_json::json!({ "name": "chashu" });
    let res = surf::post(uri).body(surf::Body::from_json(&data)?).await?;
    assert_eq!(res.status(), surf::StatusCode::Ok);
    Ok(())
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
