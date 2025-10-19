use std::fmt;

macro_rules! keywords {
    ( $( $tok:ident ),+ $(,)? ) => {
        lazy_static::lazy_static! {
            pub(crate) static ref KEYWORDS: std::collections::HashMap<String, Token> = {
                let mut m = std::collections::HashMap::new();
                $(
                    m.insert(stringify!($tok).to_lowercase(), Token::$tok);
                )+
                m
            };
        }

        pub(crate) fn keyword_to_token(ident: &str) -> Token {
            KEYWORDS.get(&ident.to_lowercase()[..])
                .cloned()
                .unwrap_or(Token::Identifier(ident.to_string()))
        }
    };
}

keywords! {
    Select, From, Where, And, Or, Not, Like, In, Between, Is, Null, True, False,
    Case, When, Then, Else, End, Order, By, Group, Having, Asc, Desc, Insert,
    Into, Values, Update, Set, Delete, Create, Table, Drop, Limit, Join,
    Inner, Outer, Full, Left, Right, Cross, Exists, Any, All, Some, On, As,
    Distinct, Union, Intersect, Except, With, Recursive, Primary, Key, Foreign,
    References, Unique, Index, View, Procedure, Function, Trigger, Database,
    Schema, Grant, Revoke, Commit, Rollback, Transaction, Begin, End, Constraint,
    Default, Check, Alter, Add, Column, Modify, Rename, To, Lock
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    // Keywords
    Select,
    From,
    Where,
    And,
    Or,
    Not,
    Like,
    In,
    Between,
    Is,
    Null,
    True,
    False,
    Case,
    When,
    Then,
    Else,
    End,
    Order,
    By,
    Group,
    Having,
    Asc,
    Desc,
    Insert,
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Drop,
    Limit,
    Join,
    Inner,
    Outer,
    Full,
    Left,
    Right,
    Cross,
    Exists,
    Any,
    All,
    Some,
    On,
    As,
    Distinct,
    Union,
    Intersect,
    Except,
    With,
    Recursive,
    Primary,
    Key,
    Foreign,
    References,
    Unique,
    Index,
    View,
    Procedure,
    Function,
    Trigger,
    Database,
    Schema,
    Grant,
    Revoke,
    Commit,
    Rollback,
    Transaction,
    Begin,
    Constraint,
    Default,
    Check,
    Alter,
    Add,
    Column,
    Modify,
    Rename,
    To,
    Lock,

    // Identifiers and literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),

    // Symbols and operators
    Star,      // *
    Comma,     // ,
    Dot,       // .
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    Eq,        // =
    Neq,       // != or <>
    Lt,        // <
    Gt,        // >
    Le,        // <=
    Ge,        // >=
    Plus,      // +
    Minus,     // -
    Slash,     // /
    Percent,   // %
    Concat,    // ||

    // End of file
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "{}", s),
            Token::StringLiteral(s) => write!(f, "'{}'", s),
            Token::NumberLiteral(n) => write!(f, "{}", n),
            Token::Star => write!(f, "*"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Eq => write!(f, "="),
            Token::Neq => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Le => write!(f, "<="),
            Token::Ge => write!(f, ">="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Concat => write!(f, "||"),
            Token::Eof => write!(f, "EOF"),
            _ => write!(f, "{:?}", self),
        }
    }
}
