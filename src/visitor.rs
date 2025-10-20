use crate::ast::Statement;

/// The visitor pattern is a good practice for creating parsers and interpreters.
/// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
pub(crate) trait Visitor {
    fn visit(&mut self) -> Result<Statement, String>;
}
