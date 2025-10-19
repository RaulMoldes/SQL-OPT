use crate::ast::*;
use crate::{parse_sql, sql_test};


sql_test!(
    test_update_basic,
    "UPDATE my_table SET col1 = 5 WHERE col2 < 3;",
    Statement::Update(UpdateStatement { table: "my_table".to_string(), set_clauses: vec![SetClause { column: "col1".to_string(), value: Expr::Number(5.0) }], where_clause: Some(Expr::BinaryOp { left: Box::new(Expr::Identifier("col2".to_string())), op: BinaryOperator::Lt, right: Box::new(Expr::Number(3.0)) }) })

);


// TODO. NEED TO ADD SUPPORT FOR UPDATE QUERIES WITH SUBQUERIES IN THE SET CLAUSE,
