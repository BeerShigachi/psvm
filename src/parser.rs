use crate::vm::Instruction;


// --- AST定義 ---
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i32),            // 整数リテラル
    Var(String),         // 変数参照
    BinOp(Box<Expr>, BinOp, Box<Expr>), // 2項演算
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let(String, Expr),   // let x = ...
    Print(Expr),         // print/logShow ...
    ExprStmt(Expr),      // 式のみの文
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Stmt>); // 複数文対応


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

// --- ASTパーサ ---
pub fn parse_expr(s: &str) -> Option<Expr> {
    let s = s.trim();
    // 2項演算（加算のみ対応、左結合）
    if let Some(pos) = s.find('+') {
        let (left, right) = s.split_at(pos);
        let left = left.trim();
        let right = &right[1..];
        return Some(Expr::BinOp(
            Box::new(parse_expr(left)?),
            BinOp::Add,
            Box::new(parse_expr(right)?),
        ));
    }
    // 整数リテラル
    if let Ok(n) = s.parse::<i32>() {
        return Some(Expr::Int(n));
    }
    // 変数参照
    if !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(Expr::Var(s.to_string()));
    }
    None
}

pub fn parse_stmt(s: &str) -> Option<Stmt> {
    let s = s.trim();
    let let_prefix = s.strip_prefix("let ");
    let print_prefix = s.strip_prefix("print ");
    let logshow_prefix = s.strip_prefix("logShow ");
    match (let_prefix, print_prefix, logshow_prefix) {
        (Some(rest), _, _) => {
            let parts: Vec<_> = rest.split('=').map(|x| x.trim()).collect();
            if parts.len() == 2 {
                Some(Stmt::Let(parts[0].to_string(), parse_expr(parts[1])?))
            } else {
                None
            }
        }
        (None, Some(rest), _) => {
            let expr = parse_expr(rest)?;
            Some(Stmt::Print(expr))
        }
        (None, None, Some(rest)) => {
            let expr = parse_expr(rest)?;
            Some(Stmt::Print(expr))
        }
        (None, None, None) => parse_expr(s).map(Stmt::ExprStmt),
    }
}

pub fn parse_program(source: &str) -> Option<Program> {
    let mut stmts = vec![];
    for line in source.lines() {
        if let Some(stmt) = parse_stmt(line) {
            stmts.push(stmt);
        } else if !line.trim().is_empty() {
            return None;
        }
    }
    Some(Program(stmts))
}

fn expr_to_instructions(expr: &Expr) -> Vec<Instruction> {
    match expr {
        Expr::Int(n) => vec![Instruction::Push(*n)],
        Expr::Var(_) => {
            // 変数参照は未実装（今後拡張: 環境から値を取得してPushするなど）
            // 今は0をPushしておく（ダミー）
            vec![Instruction::Push(0)]
        }
        Expr::BinOp(lhs, BinOp::Add, rhs) => {
            let mut code = expr_to_instructions(lhs);
            code.extend(expr_to_instructions(rhs));
            code.push(Instruction::Add);
            code
        }
        // 他の演算子は未実装
        _ => vec![],
    }
}

fn stmt_to_instructions(stmt: &Stmt) -> Vec<Instruction> {
    match stmt {
        Stmt::Let(_name, expr) => {
            // let文: 値を計算するが変数束縛は未実装
            expr_to_instructions(expr)
        }
        Stmt::Print(expr) => {
            let mut code = expr_to_instructions(expr);
            code.push(Instruction::Print);
            code
        }
        Stmt::ExprStmt(expr) => expr_to_instructions(expr),
    }
}

fn program_to_instructions(prog: &Program) -> Vec<Instruction> {
    prog.0.iter().flat_map(stmt_to_instructions).collect()
}

pub fn parse_simple_purs(source: &str) -> Vec<Instruction> {
    // ASTパーサでASTを得て命令列に変換
    if let Some(ast) = parse_program(source) {
        program_to_instructions(&ast)
    } else {
        vec![]
    }
}
