use crate::{ast::Expression, lexer::token::Token};

// Literals
struct IntegerExpr {
    value: i64,
}
impl Expression for IntegerExpr {
    fn expr() {}
}

struct StringExpr {
    value: String,
}
impl Expression for StringExpr {
    fn expr() {}
}

// Composite
struct BinaryExpr<L: Expression, R: Expression> {
    left: L,
    operator: Token,
    right: R,
}

impl<L, R> Expression for BinaryExpr<L, R>
where
    L: Expression,
    R: Expression,
 {
    fn expr() {}
}
