#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    // Identifiers and columns
    Identifier(String),
    QualifiedIdentifier {
        table: String,
        column: String,
    },
    Star,

    // Binary operations
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },

    // Unary operations
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expr>,
    },

    // Function call
    FunctionCall {
        name: String,
        args: Vec<Expr>,
        distinct: bool,
    },

    // CASE expression
    Case {
        operand: Option<Box<Expr>>,
        when_clauses: Vec<WhenClause>,
        else_clause: Option<Box<Expr>>,
    },

    // Subquery
    Subquery(Box<SelectStatement>),

    // List (for IN operator)
    List(Vec<Expr>),

    // BETWEEN
    Between {
        expr: Box<Expr>,
        negated: bool,
        low: Box<Expr>,
        high: Box<Expr>,
    },

    // EXISTS
    Exists(Box<SelectStatement>),

    // Type cast
    Cast {
        expr: Box<Expr>,
        data_type: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WhenClause {
    pub(crate) condition: Expr,
    pub(crate) result: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BinaryOperator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,

    // Logical
    And,
    Or,

    // String
    Like,
    NotLike,
    Concat,

    // Set
    In,
    NotIn,

    // NULL
    Is,
    IsNot,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SelectStatement {
    pub(crate) distinct: bool,
    pub(crate) columns: Vec<SelectItem>,
    pub(crate) from: Option<TableReference>,
    pub(crate) where_clause: Option<Expr>,
    pub(crate) group_by: Vec<Expr>,
    pub(crate) having: Option<Expr>,
    pub(crate) order_by: Vec<OrderByExpr>,
    pub(crate) limit: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SelectItem {
    Star,
    QualifiedStar(String),
    ExprWithAlias { expr: Expr, alias: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TableReference {
    Table {
        name: String,
        alias: Option<String>,
    },
    Join {
        left: Box<TableReference>,
        join_type: JoinType,
        right: Box<TableReference>,
        on: Option<Expr>,
    },
    Subquery {
        query: Box<SelectStatement>,
        alias: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OrderByExpr {
    pub(crate) expr: Expr,
    pub(crate) asc: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Statement {
    With(WithStatement),
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    CreateTable(CreateTableStatement),
    AlterTable(AlterTableStatement),
    DropTable(DropTableStatement),
    CreateIndex(CreateIndexStatement),
    Transaction(TransactionStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WithStatement {
    pub(crate) ctes: Vec<(String, SelectStatement)>,
    pub(crate) body: Box<SelectStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InsertStatement {
    pub(crate) table: String,
    pub(crate) columns: Option<Vec<String>>,
    pub(crate) values: Values,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Values {
    Values(Vec<Vec<Expr>>),
    Query(Box<SelectStatement>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UpdateStatement {
    pub(crate) table: String,
    pub(crate) set_clauses: Vec<SetClause>,
    pub(crate) where_clause: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SetClause {
    pub(crate) column: String,
    pub(crate) value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DeleteStatement {
    pub(crate) table: String,
    pub(crate) where_clause: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CreateTableStatement {
    pub(crate) table: String,
    pub(crate) columns: Vec<ColumnDef>,
    pub(crate) constraints: Vec<TableConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ColumnDef {
    pub(crate) name: String,
    pub(crate) data_type: DataType,
    pub(crate) constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DataType {
    Integer,
    BigInt,
    SmallInt,
    Decimal(Option<u8>, Option<u8>),
    Numeric(Option<u8>, Option<u8>),
    Real,
    Double,
    Varchar(Option<usize>),
    Char(Option<usize>),
    Text,
    Date,
    Time,
    Timestamp,
    Boolean,
    Json,
    Jsonb,
    Uuid,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ColumnConstraint {
    NotNull,
    Unique,
    PrimaryKey,
    ForeignKey { table: String, column: String },
    Check(Expr),
    Default(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TableConstraint {
    PrimaryKey(Vec<String>),
    Unique(Vec<String>),
    ForeignKey {
        columns: Vec<String>,
        ref_table: String,
        ref_columns: Vec<String>,
    },
    Check(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AlterTableStatement {
    pub(crate) table: String,
    pub(crate) action: AlterAction,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AlterAction {
    AddColumn(ColumnDef),
    DropColumn(String),
    AlterColumn {
        name: String,
        action: AlterColumnAction,
    },
    AddConstraint(TableConstraint),
    DropConstraint(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AlterColumnAction {
    SetDataType(DataType),
    SetDefault(Expr),
    DropDefault,
    SetNotNull,
    DropNotNull,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DropTableStatement {
    pub(crate) table: String,
    pub(crate) if_exists: bool,
    pub(crate) cascade: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CreateIndexStatement {
    pub(crate) name: String,
    pub(crate) table: String,
    pub(crate) columns: Vec<IndexColumn>,
    pub(crate) unique: bool,
    pub(crate) if_not_exists: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IndexColumn {
    pub(crate) name: String,
    pub(crate) order: Option<OrderDirection>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TransactionStatement {
    Begin,
    Commit,
    Rollback,
}
