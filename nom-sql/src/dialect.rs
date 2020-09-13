use ::{SelectStatement};
use ::{condition, ConditionTree};
use ::{ConditionExpression, Operator};
use ::{SqlQuery, JoinClause};

pub fn add_where(mut source: SelectStatement, where_add: &str) -> SqlQuery {
  let alias: Vec<_> = source.tables.iter().map(|t| t.alias.as_ref().unwrap()).collect();
  dbg!(&alias);

  let tmp = format!("{}.{}", &alias[0], where_add);
  let condition_expr = condition::condition_expr(tmp.as_bytes()).unwrap().1;

  //let joins = source.join;

  source.join.iter().for_each(|e| join_op(e));

  let parser_condition_result = match source.where_clause {
    Some(cond) => {
      ConditionExpression::LogicalOp(ConditionTree {
        operator: Operator::And,
        left: Box::new(cond),
        right: Box::new(condition_expr),
      })
    },
    None => {
      condition_expr
    },
  };

  source.where_clause = Option::from(parser_condition_result);

  SqlQuery::Select(source)
}

pub fn join_op(e: &JoinClause) {
  let cstt = &e.constraint;
  let op = &e.operator;
  let rt = &e.right;

  dbg!(cstt);
  dbg!(op);
  dbg!(rt);
}