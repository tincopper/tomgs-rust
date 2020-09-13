extern crate nom_sql;
extern crate nom;

use nom_sql::{parse_query, condition, ConditionExpression, ConditionTree, Operator, SelectStatement, JoinClause, JoinConstraint, common, JoinRightSide, Table};
use nom_sql::select;
use nom::IResult;

#[cfg(test)]
mod tests {
  use super::*;
  use nom_sql::{sql_query_by_custom, sql_query};

  #[test]
  fn edit_select_query() {
    let qstring0 = "SELECT * FROM users";
    let res0 = parse_query(qstring0);
    let query = res0.unwrap();

    println!("sql_query: {}", query);

    let select = select::selection(qstring0.trim().as_bytes());
    let mut select_statement = select.unwrap().1;
    println!("select_statement: {:#?}", select_statement);

    //let cond_expr = select_statement.where_clause.unwrap();
    //println!("where_cond_expr: {:#?}", cond_expr);

    let where_str = "where accountid=1 and name='tomgs';";
    let where_clause = select::where_clause(where_str.trim().as_bytes());
    let where_cond_expr = where_clause.unwrap().1;
    println!("where_cond_expr: {:#?}", where_cond_expr);

    let condition_str = "age=18";
    let condition_expr = condition::condition_expr(condition_str.trim().as_bytes());
    let tmp_condition = condition_expr.unwrap().1;
    println!("condition_expr: {:#?}", tmp_condition);

    let op = ConditionExpression::LogicalOp(ConditionTree {
      operator: Operator::And,
      left: Box::new(where_cond_expr),
      right: Box::new(tmp_condition),
    });

    println!("result condition: {}", op);

    select_statement.where_clause = Option::from(op);
    println!("select_statement: {}", select_statement); // SELECT * FROM users WHERE accountid = 1 AND name = 'tomgs' AND age = 18

    let cond_expr = select_statement.where_clause.unwrap();
    println!("where_cond_expr: {:#?}", cond_expr);

    panic!("");
  }

  #[test]
  pub fn test_add_where() {
    let source = "SELECT * FROM users";
    let where_cond = "accountid=123";

    let result_sql = "SELECT * FROM users WHERE accountid = 123";

    let result = add_where(source.as_bytes(), where_cond.as_bytes());
    let result = result.unwrap().1;

    println!("parser result: {} ==> {}", source, result);

    assert_eq!(result_sql, format!("{}", result));
  }

  #[test]
  pub fn test_add_where2() {
    let source = "SELECT * FROM users where name='tomgs'";
    let where_cond = "accountid=123";

    let result_sql = "SELECT * FROM users WHERE name = 'tomgs' AND accountid = 123";

    let result = add_where(source.as_bytes(), where_cond.as_bytes());
    let result = result.unwrap().1;

    println!("parser result: {} ==> {}", source, result);

    assert_eq!(result_sql, format!("{}", result));
  }

  #[test]
  pub fn test_parse() {
    //let source = "SELECT * FROM users where name='tomgs'";
    let source = "select su.fid, su.fnumber, sul.fcomment from t_sec_user as su left join t_sec_user_l as sul on su.fid = sul.fid";
    let where_cond = "accountid=123";
    let result = sql_query_by_custom(source.trim().as_bytes(), where_cond.trim());

    let query = result.unwrap().1;
    println!("sql_query: {}", query);

    let result = sql_query(source.trim().as_bytes());
    println!("sql_query: {}", result.unwrap().1);
  }

  #[test]
  pub fn test_add_join_condition() {
    //let source = "select su.fid, su.fnumber, sul.fcomment from t_sec_user as su left join t_sec_user_l as sul on su.fid = sul.fid";
    let source = "select su.fid, su.fnumber, sul.fcomment from t_sec_user as su left join t_sec_user_l as sul on su.fid = sul.fid left join t_test as tt on su.id = tt.id";
    //let cond = "su.acccountid = sul.accountid";
    let result = add_join_condition(source.trim().as_bytes());
    let result = result.unwrap().1;
    println!("parser result: {} ==> {}", source, result);
  }

  #[test]
  pub fn test_add_schema() {
    let qstring0 = "SELECT * FROM users";
    //let qstring0 = "select su.fid, su.fnumber, sul.fcomment from t_sec_user as su left join t_sec_user_l as sul on su.fid = sul.fid";
    let result = add_schema(qstring0.trim().as_bytes(), "test");

    println!("parser result: {} ==> {}", qstring0, result.unwrap().1);
  }

}

pub fn add_schema<'a>(source: &'a [u8], schema: &'a str) -> IResult<&'a [u8], SelectStatement> {
  let mut statement = select::selection(source).unwrap().1;
  let mut table_tmp: Vec<Table> = Vec::new();

  statement.clone().tables.into_iter().for_each(|mut table| {
    if None == table.schema {
      table.schema = Some(String::from(schema));
    }
    table_tmp.push(table);
  });

  statement.tables = table_tmp;

  Ok((
    "".as_bytes(),
    statement,
  ))
}

pub fn add_join_condition(source: &[u8]) -> IResult<&[u8], SelectStatement> {
  let mut statement = select::selection(source).unwrap().1;

  //let condition_expr = condition::condition_expr(cond);
  //let condition_expr = condition_expr.unwrap().1;

  //let jc_tmp = JoinConstraint::On(condition_expr);
  ////dbg!(jc_tmp);
  //println!("jc_tmp: {}", jc_tmp);

  let mut join_tmp: Vec<JoinClause> = Vec::new();

  let alias: Vec<_> = statement.tables.iter().map(|e| e.alias.as_ref().unwrap()).collect();
  let alias = &alias[0];

  statement.clone().join.into_iter().for_each(|mut jc| {
    //join_op(&jc);
    let sub_alias = match jc.clone().right {
      JoinRightSide::Table(t) => {
        t.alias.unwrap()
      },
      _ => {
        String::from("")
      },
    };

    let cond = format!("{}.accountid = {}.accountid", alias, sub_alias);

    let condition_expr = condition::condition_expr(cond.as_bytes());
    let condition_expr = condition_expr.unwrap().1;

    let jcst = match jc.constraint {
      JoinConstraint::On(cond) => {
        let on_cond = ConditionExpression::LogicalOp(ConditionTree {
          operator: Operator::And,
          left: Box::new(cond),
          right: Box::new(condition_expr),
        });
        JoinConstraint::On(on_cond)
      },
      JoinConstraint::Using(mut columns) => {
        columns.push(common::column_identifier_no_alias("accountid".trim().as_bytes()).unwrap().1);
        JoinConstraint::Using(columns)
      },
    };

    jc.constraint = jcst;
    join_tmp.push(jc);
  });

  //join_tmp.iter().for_each(|f| println!("{}", f));

  statement.join = join_tmp;
  Ok((
    "".as_bytes(),
    statement,
  ))
}

pub fn add_where<'a>(source: &'a[u8], where_add: &'a[u8]) -> IResult<&'a[u8], SelectStatement> {
  let mut statement = select::selection(source).unwrap().1;

  let condition_expr = condition::condition_expr(where_add);
  let tmp_condition = condition_expr.unwrap().1;

  let parser_condition_result = match statement.where_clause {
    Some(cond) => {
      ConditionExpression::LogicalOp(ConditionTree {
        operator: Operator::And,
        left: Box::new(cond),
        right: Box::new(tmp_condition),
      })
    },
    None => {
      tmp_condition
    },
  };

  statement.where_clause = Option::from(parser_condition_result);

  Ok((
    "".as_bytes(),
    statement,
  ))
}