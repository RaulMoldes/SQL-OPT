use crate::ast::*;
use crate::{parse_sql, sql_test};

sql_test!(
    test_create_table_basic,
    "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(255) NOT NULL);",
    Statement::CreateTable(CreateTableStatement {
        table: "users".to_string(),
        columns: vec![
            ColumnDef {
                name: "id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![ColumnConstraint::PrimaryKey]
            },
            ColumnDef {
                name: "name".to_string(),
                data_type: DataType::Varchar(Some(255)),
                constraints: vec![ColumnConstraint::NotNull]
            }
        ],
        constraints: vec![]
    })
);

sql_test!(
    test_create_table_with_foreign_key,
    "CREATE TABLE orders (id INTEGER PRIMARY KEY, user_id INTEGER REFERENCES users(id));",
    Statement::CreateTable(CreateTableStatement {
        table: "orders".to_string(),
        columns: vec![
            ColumnDef {
                name: "id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![ColumnConstraint::PrimaryKey]
            },
            ColumnDef {
                name: "user_id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![ColumnConstraint::ForeignKey {
                    table: "users".to_string(),
                    column: "id".to_string()
                }]
            }
        ],
        constraints: vec![]
    })
);

sql_test!(
    test_create_table_with_check_constraint,
    "CREATE TABLE products (id INTEGER, price DECIMAL(10, 2) CHECK (price > 0));",
    Statement::CreateTable(CreateTableStatement {
        table: "products".to_string(),
        columns: vec![
            ColumnDef {
                name: "id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![]
            },
            ColumnDef {
                name: "price".to_string(),
                data_type: DataType::Decimal(Some(10), Some(2)),
                constraints: vec![ColumnConstraint::Check(Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("price".to_string())),
                    op: BinaryOperator::Gt,
                    right: Box::new(Expr::Number(0.0))
                })]
            }
        ],
        constraints: vec![]
    })
);

sql_test!(
    test_create_table_with_default,
    "CREATE TABLE posts (id INTEGER, status VARCHAR(20) DEFAULT 'draft');",
    Statement::CreateTable(CreateTableStatement {
        table: "posts".to_string(),
        columns: vec![
            ColumnDef {
                name: "id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![]
            },
            ColumnDef {
                name: "status".to_string(),
                data_type: DataType::Varchar(Some(20)),
                constraints: vec![ColumnConstraint::Default(Expr::String("draft".to_string()))]
            }
        ],
        constraints: vec![]
    })
);

sql_test!(
    test_create_table_with_table_constraints,
    "CREATE TABLE order_items (order_id INTEGER, product_id INTEGER, PRIMARY KEY (order_id, product_id));",
    Statement::CreateTable(CreateTableStatement {
        table: "order_items".to_string(),
        columns: vec![
            ColumnDef {
                name: "order_id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![]
            },
            ColumnDef {
                name: "product_id".to_string(),
                data_type: DataType::Integer,
                constraints: vec![]
            }
        ],
        constraints: vec![TableConstraint::PrimaryKey(vec![
            "order_id".to_string(),
            "product_id".to_string()
        ])]
    })
);

sql_test!(
    test_alter_table_add_column,
    "ALTER TABLE users ADD COLUMN email VARCHAR(255);",
    Statement::AlterTable(AlterTableStatement {
        table: "users".to_string(),
        action: AlterAction::AddColumn(ColumnDef {
            name: "email".to_string(),
            data_type: DataType::Varchar(Some(255)),
            constraints: vec![]
        })
    })
);

sql_test!(
    test_alter_table_drop_column,
    "ALTER TABLE users DROP COLUMN email;",
    Statement::AlterTable(AlterTableStatement {
        table: "users".to_string(),
        action: AlterAction::DropColumn("email".to_string())
    })
);

sql_test!(
    test_alter_table_alter_column_type,
    "ALTER TABLE products ALTER COLUMN price DECIMAL(12, 2);",
    Statement::AlterTable(AlterTableStatement {
        table: "products".to_string(),
        action: AlterAction::AlterColumn(AlterColumnStatement {
            name: "price".to_string(),
            action: AlterColumnAction::SetDataType(DataType::Decimal(Some(12), Some(2)))
        })
    })
);

sql_test!(
    test_alter_table_set_default,
    "ALTER TABLE orders ALTER COLUMN status SET DEFAULT 'pending';",
    Statement::AlterTable(AlterTableStatement {
        table: "orders".to_string(),
        action: AlterAction::AlterColumn(AlterColumnStatement {
            name: "status".to_string(),
            action: AlterColumnAction::SetDefault(Expr::String("pending".to_string()))
        })
    })
);

sql_test!(
    test_alter_table_drop_default,
    "ALTER TABLE orders ALTER COLUMN status DROP DEFAULT;",
    Statement::AlterTable(AlterTableStatement {
        table: "orders".to_string(),
        action: AlterAction::AlterColumn(AlterColumnStatement {
            name: "status".to_string(),
            action: AlterColumnAction::DropDefault
        })
    })
);

sql_test!(
    test_drop_table_simple,
    "DROP TABLE users;",
    Statement::DropTable(DropTableStatement {
        table: "users".to_string(),
        if_exists: false,
        cascade: false
    })
);

sql_test!(
    test_drop_table_if_exists,
    "DROP TABLE IF EXISTS temp_data;",
    Statement::DropTable(DropTableStatement {
        table: "temp_data".to_string(),
        if_exists: true,
        cascade: false
    })
);

sql_test!(
    test_drop_table_cascade,
    "DROP TABLE orders CASCADE;",
    Statement::DropTable(DropTableStatement {
        table: "orders".to_string(),
        if_exists: false,
        cascade: true
    })
);

sql_test!(
    test_create_index_simple,
    "CREATE INDEX idx_user_email ON users (email);",
    Statement::CreateIndex(CreateIndexStatement {
        name: "idx_user_email".to_string(),
        table: "users".to_string(),
        columns: vec![IndexColumn {
            name: "email".to_string(),
            order: None
        }],
        unique: false,
        if_not_exists: false
    })
);

sql_test!(
    test_create_unique_index,
    "CREATE UNIQUE INDEX idx_user_username ON users (username);",
    Statement::CreateIndex(CreateIndexStatement {
        name: "idx_user_username".to_string(),
        table: "users".to_string(),
        columns: vec![IndexColumn {
            name: "username".to_string(),
            order: None
        }],
        unique: true,
        if_not_exists: false
    })
);

sql_test!(
    test_create_index_multi_column,
    "CREATE INDEX idx_orders_composite ON orders (user_id ASC, created_at DESC);",
    Statement::CreateIndex(CreateIndexStatement {
        name: "idx_orders_composite".to_string(),
        table: "orders".to_string(),
        columns: vec![
            IndexColumn {
                name: "user_id".to_string(),
                order: Some(OrderDirection::Asc)
            },
            IndexColumn {
                name: "created_at".to_string(),
                order: Some(OrderDirection::Desc)
            }
        ],
        unique: false,
        if_not_exists: false
    })
);

sql_test!(
    test_begin_transaction,
    "BEGIN;",
    Statement::Transaction(TransactionStatement::Begin)
);

sql_test!(
    test_begin_transaction_explicit,
    "BEGIN TRANSACTION;",
    Statement::Transaction(TransactionStatement::Begin)
);

sql_test!(
    test_commit_transaction,
    "COMMIT;",
    Statement::Transaction(TransactionStatement::Commit)
);

sql_test!(
    test_rollback_transaction,
    "ROLLBACK;",
    Statement::Transaction(TransactionStatement::Rollback)
);

sql_test!(
    test_various_data_types,
    "CREATE TABLE test_types (
        col_int INTEGER,
        col_bigint BIGINT,
        col_smallint SMALLINT,
        col_decimal DECIMAL(10, 2),
        col_numeric NUMERIC(8, 4),
        col_real REAL,
        col_double DOUBLE,
        col_varchar VARCHAR(100),
        col_char CHAR(10),
        col_text TEXT,
        col_date DATE,
        col_time TIME,
        col_timestamp TIMESTAMP,
        col_boolean BOOLEAN,
        col_json JSON,
        col_jsonb JSONB,
        col_uuid UUID
    );",
    Statement::CreateTable(CreateTableStatement {
        table: "test_types".to_string(),
        columns: vec![
            ColumnDef {
                name: "col_int".to_string(),
                data_type: DataType::Integer,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_bigint".to_string(),
                data_type: DataType::BigInt,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_smallint".to_string(),
                data_type: DataType::SmallInt,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_decimal".to_string(),
                data_type: DataType::Decimal(Some(10), Some(2)),
                constraints: vec![]
            },
            ColumnDef {
                name: "col_numeric".to_string(),
                data_type: DataType::Numeric(Some(8), Some(4)),
                constraints: vec![]
            },
            ColumnDef {
                name: "col_real".to_string(),
                data_type: DataType::Real,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_double".to_string(),
                data_type: DataType::Double,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_varchar".to_string(),
                data_type: DataType::Varchar(Some(100)),
                constraints: vec![]
            },
            ColumnDef {
                name: "col_char".to_string(),
                data_type: DataType::Char(Some(10)),
                constraints: vec![]
            },
            ColumnDef {
                name: "col_text".to_string(),
                data_type: DataType::Text,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_date".to_string(),
                data_type: DataType::Date,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_time".to_string(),
                data_type: DataType::Time,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_timestamp".to_string(),
                data_type: DataType::Timestamp,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_boolean".to_string(),
                data_type: DataType::Boolean,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_json".to_string(),
                data_type: DataType::Json,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_jsonb".to_string(),
                data_type: DataType::Jsonb,
                constraints: vec![]
            },
            ColumnDef {
                name: "col_uuid".to_string(),
                data_type: DataType::Uuid,
                constraints: vec![]
            }
        ],
        constraints: vec![]
    })
);
