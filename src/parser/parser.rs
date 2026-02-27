use crate::ast::ast::{Expr, Stmt, BinOp, Program};
use crate::codegen::codegen::program_to_instructions;
use crate::vm::Instruction;

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
            let parts: Vec<_> = rest.split('=') .map(|x| x.trim()).collect();
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

pub fn parse_simple_purs(source: &str) -> Vec<Instruction> {
    // ASTパーサでASTを得て命令列に変換
    if let Some(ast) = parse_program(source) {
        program_to_instructions(&ast)
    } else {
        vec![]
    }
}
