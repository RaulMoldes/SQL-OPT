use crate::ast::*;

pub(crate) trait SQLParser {
    fn parse(&mut self) -> Result<Statement, String>;
}

/// The visitor pattern is a good practice for creating parsers and interpreters.
/// https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
pub(crate) trait StatementVisitor {
    fn visit_select_statement(&mut self) -> Result<SelectStatement, String>;
    fn visit_with_statement(&mut self) -> Result<WithStatement, String>;
    fn visit_insert_statement(&mut self) -> Result<InsertStatement, String>;
    fn visit_update_statement(&mut self) -> Result<UpdateStatement, String>;
    fn visit_delete_statement(&mut self) -> Result<DeleteStatement, String>;
    fn visit_create_table_statement(&mut self) -> Result<CreateTableStatement, String>;
    fn visit_create_index_statement(&mut self) -> Result<CreateIndexStatement, String>;
    fn visit_alter_statement(&mut self) -> Result<AlterTableStatement, String>;
    fn visit_drop_statement(&mut self) -> Result<DropTableStatement, String>;
    fn visit_transaction_statement(&mut self) -> Result<TransactionStatement, String>;
}

pub(crate) trait ItemVisitor {
    fn visit_select_list(&mut self) -> Result<Vec<SelectItem>, String>;

    fn visit_table_ref(&mut self) -> Result<TableReference, String>;
    fn visit_data_type(&mut self) -> Result<DataType, String>;
    fn visit_column_def(&mut self) -> Result<ColumnDef, String>;
    fn visit_column_constraints(&mut self) -> Result<Vec<ColumnConstraint>, String>;
    fn visit_table_constraint(&mut self) -> Result<TableConstraint, String>;
}
