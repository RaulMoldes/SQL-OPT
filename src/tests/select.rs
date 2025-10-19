use crate::ast::*;
use crate::{parse_sql, sql_test};

sql_test!(
    select_star_from_table,
    "SELECT * FROM my_table;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::Star],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None,
    })
);

sql_test!(
    select_star_from_table_with_filter,
    "SELECT * FROM my_table WHERE COL1 > 2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::Star],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: Some(Expr::BinaryOp {
            left: Box::new(Expr::Identifier("COL1".to_string())),
            op: BinaryOperator::Gt,
            right: Box::new(Expr::Number(2.0))
        }),
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None,
    })
);

sql_test!(
    select_star_from_table_with_multi_filter,
    "SELECT * FROM my_table WHERE COL1 > 2 AND COL2 < 3;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::Star],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: Some(Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("COL1".to_string())),
                op: BinaryOperator::Gt,
                right: Box::new(Expr::Number(2.0))
            }),
            op: BinaryOperator::And,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("COL2".to_string())),
                op: BinaryOperator::Lt,
                right: Box::new(Expr::Number(3.0))
            })
        }),
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None,
    })
);

sql_test!(
    select_single_column,
    "SELECT col1 FROM my_table;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::Identifier("col1".to_string()),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_distinct_single_column,
    "SELECT DISTINCT col1 FROM my_table;",
    Statement::Select(SelectStatement {
        distinct: true,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::Identifier("col1".to_string()),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_multiple_columns,
    "SELECT (col1, col2, col3) FROM my_table;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_group_by,
    "SELECT (MAX(col1), col2) FROM my_table GROUP BY col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::FunctionCall {
                    name: "MAX".to_string(),
                    args: vec![Expr::Identifier("col1".to_string())],
                    distinct: false
                },
                Expr::Identifier("col2".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![Expr::Identifier("col2".to_string())],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_group_by_having_clause,
    "SELECT (MAX(col1), col2) FROM my_table GROUP BY col2 HAVING COUNT(*) < 1;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::FunctionCall {
                    name: "MAX".to_string(),
                    args: vec![Expr::Identifier("col1".to_string())],
                    distinct: false
                },
                Expr::Identifier("col2".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "my_table".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![Expr::Identifier("col2".to_string())],
        having: Some(Expr::BinaryOp {
            left: Box::new(Expr::FunctionCall {
                name: "COUNT".to_string(),
                args: vec![Expr::Star],
                distinct: false
            }),
            op: BinaryOperator::Lt,
            right: Box::new(Expr::Number(1.0))
        }),
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_casting_op,
    "SELECT CAST(3, STRING);",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::FunctionCall {
                name: "CAST".to_string(),
                args: vec![Expr::Number(3.0), Expr::Identifier("STRING".to_string())],
                distinct: false
            },
            alias: None
        }],
        from: None,
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_order_by_limit,
    "SELECT (col1, col2, col3, col4) FROM table1 ORDER BY col1 LIMIT 100;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "table1".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![OrderByExpr {
            expr: Expr::Identifier("col1".to_string()),
            asc: true
        }],
        limit: Some(100)
    })
);

sql_test!(
    select_order_by_desc,
    "SELECT (col1, col2, col3, col4) FROM table1 ORDER BY col1 DESC LIMIT 100;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Table {
            name: "table1".to_string(),
            alias: None
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![OrderByExpr {
            expr: Expr::Identifier("col1".to_string()),
            asc: false
        }],
        limit: Some(100)
    })
);

sql_test!(
    select_simple_join,
    "SELECT (col1, col2, col3, col4) FROM table1 a JOIN table2 b on a.col1 = b.col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Table {
                name: "table1".to_string(),
                alias: Some("a".to_string())
            }),
            join_type: JoinType::Inner,
            right: Box::new(TableReference::Table {
                name: "table2".to_string(),
                alias: Some("b".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col1".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "b".to_string(),
                    column: "col2".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_inner_join,
    "SELECT (col1, col2, col3, col4) FROM table1 a INNER JOIN table2 b on a.col1 = b.col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Table {
                name: "table1".to_string(),
                alias: Some("a".to_string())
            }),
            join_type: JoinType::Inner,
            right: Box::new(TableReference::Table {
                name: "table2".to_string(),
                alias: Some("b".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col1".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "b".to_string(),
                    column: "col2".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_left_join,
    "SELECT (col1, col2, col3, col4) FROM table1 a LEFT JOIN table2 b on a.col1 = b.col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Table {
                name: "table1".to_string(),
                alias: Some("a".to_string())
            }),
            join_type: JoinType::Left,
            right: Box::new(TableReference::Table {
                name: "table2".to_string(),
                alias: Some("b".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col1".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "b".to_string(),
                    column: "col2".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_right_join,
    "SELECT (col1, col2, col3, col4) FROM table1 a RIGHT JOIN table2 b on a.col1 = b.col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Table {
                name: "table1".to_string(),
                alias: Some("a".to_string())
            }),
            join_type: JoinType::Right,
            right: Box::new(TableReference::Table {
                name: "table2".to_string(),
                alias: Some("b".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col1".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "b".to_string(),
                    column: "col2".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_full_join,
    "SELECT (col1, col2, col3, col4) FROM table1 a FULL JOIN table2 b on a.col1 = b.col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Table {
                name: "table1".to_string(),
                alias: Some("a".to_string())
            }),
            join_type: JoinType::Full,
            right: Box::new(TableReference::Table {
                name: "table2".to_string(),
                alias: Some("b".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col1".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "b".to_string(),
                    column: "col2".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_cross_join,
    "SELECT (col1, col2, col3, col4) FROM table1 a CROSS JOIN table2 b on a.col1 = b.col2;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Table {
                name: "table1".to_string(),
                alias: Some("a".to_string())
            }),
            join_type: JoinType::Cross,
            right: Box::new(TableReference::Table {
                name: "table2".to_string(),
                alias: Some("b".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col1".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "b".to_string(),
                    column: "col2".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_multijoin,
    "SELECT (col1, col2, col3, col4) FROM table1 a JOIN table2 b on a.col1 = b.col2 JOIN table3 c on c.col3 = a.col3;",
    Statement::Select(SelectStatement {
        distinct: false,
        columns: vec![SelectItem::ExprWithAlias {
            expr: Expr::List(vec![
                Expr::Identifier("col1".to_string()),
                Expr::Identifier("col2".to_string()),
                Expr::Identifier("col3".to_string()),
                Expr::Identifier("col4".to_string())
            ]),
            alias: None
        }],
        from: Some(TableReference::Join {
            left: Box::new(TableReference::Join {
                left: Box::new(TableReference::Table {
                    name: "table1".to_string(),
                    alias: Some("a".to_string())
                }),
                join_type: JoinType::Inner,
                right: Box::new(TableReference::Table {
                    name: "table2".to_string(),
                    alias: Some("b".to_string())
                }),
                on: Some(Expr::BinaryOp {
                    left: Box::new(Expr::QualifiedIdentifier {
                        table: "a".to_string(),
                        column: "col1".to_string()
                    }),
                    op: BinaryOperator::Eq,
                    right: Box::new(Expr::QualifiedIdentifier {
                        table: "b".to_string(),
                        column: "col2".to_string()
                    })
                })
            }),
            join_type: JoinType::Inner,
            right: Box::new(TableReference::Table {
                name: "table3".to_string(),
                alias: Some("c".to_string())
            }),
            on: Some(Expr::BinaryOp {
                left: Box::new(Expr::QualifiedIdentifier {
                    table: "c".to_string(),
                    column: "col3".to_string()
                }),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::QualifiedIdentifier {
                    table: "a".to_string(),
                    column: "col3".to_string()
                })
            })
        }),
        where_clause: None,
        group_by: vec![],
        having: None,
        order_by: vec![],
        limit: None
    })
);

sql_test!(
    select_subquery_simple,
    "WITH t1 AS (SELECT * FROM my_table) SELECT * FROM t1;",
    Statement::With(WithStatement {
        ctes: vec![(
            "t1".to_string(),
            SelectStatement {
                distinct: false,
                columns: vec![SelectItem::Star],
                from: Some(TableReference::Table {
                    name: "my_table".to_string(),
                    alias: None
                }),
                where_clause: None,
                group_by: vec![],
                having: None,
                order_by: vec![],
                limit: None
            }
        )],
        body: Box::new(SelectStatement {
            distinct: false,
            columns: vec![SelectItem::Star],
            from: Some(TableReference::Table {
                name: "t1".to_string(),
                alias: None
            }),
            where_clause: None,
            group_by: vec![],
            having: None,
            order_by: vec![],
            limit: None
        })
    })
);

sql_test!(
    select_multiple_subqueries,
    "WITH t1 as (SELECT * FROM my_table), t2 as (SELECT * FROM my_other_table) SELECT * FROM t1 JOIN t2 ON t1 = t2;",
    Statement::With(WithStatement {
        ctes: vec![
            (
                "t1".to_string(),
                SelectStatement {
                    distinct: false,
                    columns: vec![SelectItem::Star],
                    from: Some(TableReference::Table {
                        name: "my_table".to_string(),
                        alias: None
                    }),
                    where_clause: None,
                    group_by: vec![],
                    having: None,
                    order_by: vec![],
                    limit: None
                }
            ),
            (
                "t2".to_string(),
                SelectStatement {
                    distinct: false,
                    columns: vec![SelectItem::Star],
                    from: Some(TableReference::Table {
                        name: "my_other_table".to_string(),
                        alias: None
                    }),
                    where_clause: None,
                    group_by: vec![],
                    having: None,
                    order_by: vec![],
                    limit: None
                }
            )
        ],
        body: Box::new(SelectStatement {
            distinct: false,
            columns: vec![SelectItem::Star],
            from: Some(TableReference::Join {
                left: Box::new(TableReference::Table {
                    name: "t1".to_string(),
                    alias: None
                }),
                join_type: JoinType::Inner,
                right: Box::new(TableReference::Table {
                    name: "t2".to_string(),
                    alias: None
                }),
                on: Some(Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("t1".to_string())),
                    op: BinaryOperator::Eq,
                    right: Box::new(Expr::Identifier("t2".to_string()))
                })
            }),
            where_clause: None,
            group_by: vec![],
            having: None,
            order_by: vec![],
            limit: None
        })
    })
);
