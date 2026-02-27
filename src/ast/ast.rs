// --- AST型定義 ---
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i32),
    Var(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
    ExprStmt(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
