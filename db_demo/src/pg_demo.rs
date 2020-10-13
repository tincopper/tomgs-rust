use postgres::{Client, NoTls};

struct Person {
  id: i32,
  name: String,
  data: Option<Vec<u8>>
}

/// 测试pg数据库操作
pub fn pg_exer() {
  let mut client = Client::connect("host=localhost user=jdy port=5432 password=Jdy#2019 dbname=private_cloud_dev", NoTls).unwrap();

  for row in &client.query("SELECT id, name, data FROM person", &[]).unwrap() {
    let person = Person {
      id: row.get(0),
      name: row.get(1),
      data: row.get(2)
    };
    println!("Found person {}", person.name);
  }
}