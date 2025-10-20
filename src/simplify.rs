use crate::ast::Statement;
use crate::parser::Parser;
use crate::visitor::Visitor;

pub(crate) trait Simplify {
    fn simplify(&mut self) -> Result<(), String> {
        Ok(())
    }
}


pub(crate) struct Simplifyer {
    parser: Parser,
}

impl Simplifyer {
    pub(crate) fn new(parser: Parser) -> Self {
        Self {
            parser
        }
    }
}

impl Visitor for Simplifyer {
    fn visit(&mut self) -> Result<Statement, String> {
        let mut stmt = self.parser.visit()?;
        stmt.simplify()?;
        Ok(stmt)
    }
}
