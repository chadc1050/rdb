pub struct AST<'a> {
    stmts: Vec<StatementKind<'a>>,
}

impl<'a> AST<'a> {
    pub fn new() -> Self {
        AST { stmts: Vec::new() }
    }

    pub fn append_stmt(&mut self, stmt: StatementKind<'a>) {
        self.stmts.push(stmt);
    }
}

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

pub struct BlockStmt<'a> {
    stmts: Vec<StatementKind<'a>>,
}

pub struct SelectStmt<'a> {
    table: &'a str,
    columns: Vec<&'a str>,
    top: Option<usize>,
    limit: Option<usize>,
}

pub struct UpdateStmt<'a> {
    table: &'a str,
}

pub struct InsertStmt<'a> {
    table: &'a str,
}

pub struct DeleteStmt<'a> {
    table: &'a str,
}

pub struct CreateTableStmt<'a> {
    table: &'a str,
}

pub struct SelectClause {}

pub struct FromClause {}

pub struct WhereClause {}

pub struct GroupByClause {}

pub struct HavingClause {}

pub struct OrderByClause {}
