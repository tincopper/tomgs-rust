use postgres::{Client, NoTls, SimpleQueryMessage};
use postgres::types::ToSql;

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
      println!("error: {:?}", e);
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

//  172.20.183.155
pub fn pg_parse_query(sql: &str, params: &[&(dyn ToSql + Sync)]) {
  let mut client = Client::connect("host=172.20.183.155 user=jdy port=5432 password=Jdy#2019 dbname=private_cloud_dev", NoTls).unwrap();

  // 简单查询
  let result = client.query(sql, params);
  match result {
    Ok(r) => {
      for row in r {
        let id: String = row.get("id");
        println!("Found data {:?}", id);
      }
    },
    Err(e) => {
      println!("error: {:?}", e);
    }
  }

}

#[test]
pub fn test() {
  pg_exer();
}

#[test]
pub fn test2() {
  // /*schema|where_condition|join_condition|order_by_condition|group_by_condtion|having_codition*/
  let sql = "/*public|id='7aea0712-9f13-4c50-9eb7-906cf4ea55da'||||*/select * from users t;";
  pg_simple_query(sql);
}

#[test]
pub fn test3() {
  let sql = "select * from public.users t;";
  pg_simple_query(sql);
}

#[test]
pub fn test4() {
  // /*schema|where_condition|join_condition|order_by_condition|group_by_condtion|having_codition*/
  //let sql = "/*public|id='7aea0712-9f13-4c50-9eb7-906cf4ea55da'||id||*/select * from users t;";
  let sql = "/*kd_1600849898390490952|accountid='123abc-ddd'|accountid,accountname|||*/
    select su.fid, su.fnumber, sul.fcomment from t_sec_user as su
    left join t_sec_user_l as sul on su.fid = sul.fid
    left join t_other_table as tt on su.id = tt.id;";
  pg_simple_query(sql);
}

#[test]
pub fn test5() {
  let sql = "select * from public.users t where name = $1";
  let name = "tomgs";
  pg_parse_query(sql, &[&name]);
}

#[test]
pub fn test51() {
  let sql = "select * from public.users t where name = $1 and id = $2";
  let name = "tomgs";
  let id = "be8bfc5a-8c06-4816-8015-3c6924e635ee";
  pg_parse_query(sql, &[&name, &id]);
}

#[test]
pub fn test6() {
  let sql = "select * from public.users t";
  pg_parse_query(sql, &[]);
}

#[test]
pub fn test7() {
  let sql = "/*public|id='7aea0712-9f13-4c50-9eb7-906cf4ea55da'||||*/select * from public.users t where name = $1";
  let name = "tomgs";
  pg_parse_query(sql, &[&name]);
}

#[test]
pub fn test8() {
  let sql = "/*public|id='7aea0712-9f13-4c50-9eb7-906cf4ea55da'||||*/select * from public.users t";
  pg_parse_query(sql, &[]);
}

#[test]
pub fn base() {
  let q = b'B';
  println!("{}", q);

  // 116 84 90 69
  let s = String::from_utf8_lossy(&[49 as u8]);
  println!("{}", s);

  for i in 0..1 {
    println!("i: {}", i);
  }
}