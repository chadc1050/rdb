pub struct AST<'a> {
    pub stmts: Vec<StatementKind<'a>>,
}

impl<'a> AST<'a> {
    pub fn new() -> Self {
        AST { stmts: Vec::new() }
    }

    pub fn append_stmt(&mut self, stmt: StatementKind<'a>) {
        self.stmts.push(stmt);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind<'a> {
    Block(BlockStmt<'a>),
    Select(SelectStmt<'a>),
    Update(UpdateStmt<'a>),
    Insert(InsertStmt<'a>),
    Delete(DeleteStmt<'a>),
    CreateTable(CreateTableStmt<'a>),
    Commit,
    Rollback,
    Grant,
    Revoke,
}

pub enum ExprKind {
    Identifier,
    Literal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStmt<'a> {
    stmts: Vec<StatementKind<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStmt<'a> {
    table: &'a str,
    columns: Vec<&'a str>,
    top: Option<usize>,
    limit: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStmt<'a> {
    table: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertStmt<'a> {
    table: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteStmt<'a> {
    table: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateTableStmt<'a> {
    table: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct FromClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupByClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct HavingClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderByClause {}
