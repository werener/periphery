use crate::ast::{Expression, Statement};

struct BlockStmt<S: Statement> {
    body: Vec<S>,
}
impl<S> Statement for BlockStmt<S>
where
    S: Statement,
{
    fn stmt() {}
}

struct ExpressionStmt<E: Expression> {
    expression: Vec<E>,
}
impl<E> Statement for ExpressionStmt<E>
where
    E: Expression,
{
    fn stmt() {}
}
