pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

// 定义trait
pub trait Summary {
  fn summary(&self) -> String;
}

// 实现trait
impl Summary for NewsArticle {
  fn summary(&self) -> String {
    format!("{}, by {}({})", self.headline, self.author, self.location)
  }
}

impl Summary for Tweet {
  fn summary(&self) -> String {
    format!("{}: {}.", self.username, self.content)
  }
}

// 调用
pub fn test_trait() {
  let t = Tweet {
    username: String::from("tomgs"),
    content: String::from("hello"),
    reply: false,
    retweet: false
  };

  let res = t.summary();
  println!("new tweet: {}.", res);
}

/// 高级trait
