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
pub struct DatasetReference<'a> {
    pub schema: Option<&'a str>,
    pub dataset: Option<&'a str>,
}

impl<'a> DatasetReference<'a> {
    pub fn new(dataset: &'a str) -> Self {
        DatasetReference {
            schema: None,
            dataset: Some(dataset),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectReference<'a> {
    pub dataset: Option<DatasetReference<'a>>,
    pub obj: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectStmt<'a> {
    pub select_clause: SelectClause<'a>,
    pub from_clause: FromClause<'a>,
    pub where_clause: Option<WhereClause>,
    pub group_by_clause: Option<GroupByClause>,
    pub having_clause: Option<HavingClause>,
    pub order_by_clause: Option<OrderByClause>,
    pub limit_clause: Option<LimitClause>,
}

impl<'a> SelectStmt<'a> {
    pub fn new(select: SelectClause<'a>, from: FromClause<'a>) -> Self {
        SelectStmt {
            select_clause: select,
            from_clause: from,
            where_clause: None,
            group_by_clause: None,
            having_clause: None,
            order_by_clause: None,
            limit_clause: None,
        }
    }
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
pub struct SelectClause<'a> {
    pub selected: Vec<SelectItemKind<'a>>,
}

impl<'a> SelectClause<'a> {
    pub fn all() -> Self {
        SelectClause {
            selected: vec![SelectItemKind::All],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectItemKind<'a> {
    All,
    Identifier(ObjectReference<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FromClause<'a> {
    pub from: Vec<FromItemKind<'a>>,
}

impl<'a> FromClause<'a> {
    pub fn new() -> Self {
        FromClause { from: Vec::new() }
    }

    pub fn table(table: &'a str) -> Self {
        FromClause {
            from: vec![FromItemKind::Dataset(DatasetReference::new(table))],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.from.len() == 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FromItemKind<'a> {
    Dataset(DatasetReference<'a>),
    Join(JoinClause),
}

#[derive(Clone, Debug, PartialEq)]
pub struct JoinClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct WhereClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupByClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct HavingClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderByClause {}

#[derive(Clone, Debug, PartialEq)]
pub struct LimitClause {}
