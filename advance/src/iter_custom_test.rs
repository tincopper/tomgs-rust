/// 自定义迭代器
struct Counter {
  count: u32,
}

impl Counter {
  pub fn new() -> Counter {
    Counter {
      count: 0,
    }
  }
}

/// 实现迭代器
impl Iterator for Counter {
  // 因为Counter里面的存储的类型是u32，所有Item即为u32，和Java里面Iterator中是泛型一样的意思
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    // 这个就是返回元素的具体逻辑
    self.count += 1;

    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }

  }
}

#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2)); //2 = 1 * 2
  assert_eq!(counter.next(), Some(3)); //6 = 2 * 3
  assert_eq!(counter.next(), Some(4)); //12 = 3 * 4
  assert_eq!(counter.next(), Some(5)); //20 = 4 * 5
  assert_eq!(counter.next(), None);
}

#[test]
fn calling_other_iter_methods() {
  let result: u32 = Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum();
  assert_eq!(result, 18);
}