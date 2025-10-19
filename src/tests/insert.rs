use crate::ast::*;
use crate::{parse_sql, sql_test};


sql_test!(
    test_insert_basic,
    "INSERT INTO my_table(col1, col2) VALUES (1, 2);",
    Statement::Insert(InsertStatement { table: "my_table".to_string(), columns: Some(vec!["col1".to_string(), "col2".to_string()]), values: Values::Values(vec![vec![Expr::Number(1.0), Expr::Number(2.0)]]) })

);


sql_test!(
    test_insert_with_select,
    "INSERT INTO my_table(col1, col2) SELECT (col1, col2) FROM my_other_table;",
    Statement::Insert(InsertStatement { table: "my_table".to_string(), columns: Some(vec!["col1".to_string(), "col2".to_string()]), values: Values::Query(Box::new(SelectStatement { distinct: false, columns: vec![SelectItem::ExprWithAlias { expr: Expr::List(vec![Expr::Identifier("col1".to_string()), Expr::Identifier("col2".to_string())]), alias: None }], from: Some(TableReference::Table { name: "my_other_table".to_string(), alias: None }), where_clause: None, group_by: vec![], having: None, order_by: vec![], limit: None })) })

);
