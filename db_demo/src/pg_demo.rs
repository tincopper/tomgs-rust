use postgres::{Client, NoTls, SimpleQueryMessage};

struct Person {
  id: i32,
  name: String,
  data: Option<Vec<u8>>
}

/// 测试pg数据库操作
/// https://docs.rs/postgres/0.15.2/postgres/
pub fn pg_exer() {
  let mut client = Client::connect("host=localhost user=jdy port=5432 password=Jdy#2019 dbname=private_cloud_dev", NoTls).unwrap();

  // 参数化查询
  let result = &client.query("SELECT id, name, data FROM person", &[]);
  match result {
    Ok(r) => {
      for row in r {
        let person = Person {
          id: row.get(0),
          name: row.get(1),
          data: row.get(2)
        };
        println!("Found person {}", person.name);
      }
    },
    Err(e) => {
      println!("error: {}", e);
    }
  }
}

pub fn pg_simple_query(sql: &str) {
  let mut client = Client::connect("host=localhost user=jdy port=5432 password=Jdy#2019 dbname=private_cloud_dev", NoTls).unwrap();

  // 简单查询
  let result = &client.simple_query(sql);
  match result {
    Ok(r) => {
      for row in r {
        match row {
          SimpleQueryMessage::Row(query_row) => {
            /*let person = Person {
              id: query_row.get(0).unwrap().parse::<i32>().unwrap(),
              name: query_row.get(1).unwrap().to_string(),
              data: Some(vec![1, 2, 3]),
            };*/
            println!("Found data {:?}", query_row.get(0).unwrap().parse::<String>().unwrap());
          },
          _ => {}
        }
      }
    },
    Err(e) => {
      println!("error: {}", e);
    }
  }

}

#[test]
pub fn test() {
  pg_exer();
}

#[test]
pub fn test2() {
  let sql = "/*|id='7aea0712-9f13-4c50-9eb7-906cf4ea55da'||||*/select * from public.users t;";
  pg_simple_query(sql);
}

#[test]
pub fn test3() {
  let sql = "select * from public.users t;";
  pg_simple_query(sql);
}

#[test]
pub fn base() {
  let q = b'Q';
  println!("{}", q);

  let s = String::from_utf8_lossy(&[80 as u8]);
  println!("{}", s);
}