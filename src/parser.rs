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
    // let文
    if let Some(rest) = s.strip_prefix("let ") {
        let parts: Vec<_> = rest.split('=').map(|x| x.trim()).collect();
        if parts.len() == 2 {
            let var = parts[0].to_string();
            let expr = parse_expr(parts[1])?;
            return Some(Stmt::Let(var, expr));
        }
    }
    // print/logShow文
    if let Some(rest) = s.strip_prefix("print ") {
        let expr = parse_expr(rest)?;
        return Some(Stmt::Print(expr));
    }
    if let Some(rest) = s.strip_prefix("logShow ") {
        let expr = parse_expr(rest)?;
        return Some(Stmt::Print(expr));
    }
    // 式文
    parse_expr(s).map(Stmt::ExprStmt)
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Stmt>); // 複数文対応

use crate::vm::Instruction;
use std::collections::HashMap;

/// キーワード定義
#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Let,
    LogShow,
}

/// 演算子定義
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

/// オペランド（変数名や値）
#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Var(String),
    Value(i32),
}

/// キーワード文字列→Keyword
pub fn parse_keyword(s: &str) -> Option<Keyword> {
    match s {
        "let" => Some(Keyword::Let),
        "logShow" => Some(Keyword::LogShow),
        _ => None,
    }
}

/// 演算子文字列→Operator
pub fn parse_operator(s: &str) -> Option<Operator> {
    match s {
        "+" => Some(Operator::Add),
        "-" => Some(Operator::Sub),
        "*" => Some(Operator::Mul),
        "/" => Some(Operator::Div),
        _ => None,
    }
}

pub enum ParseResult {
    Assignment(String, i32),
    Instructions(Vec<Instruction>),
    None,
}

#[derive(Default)]
pub struct ParserEnv {
    pub vars: HashMap<String, i32>,
}

pub enum ParserError {
    InvalidSyntax,
    UnknownVariable(String),
    Other(String),
}

pub fn parse_simple_purs(source: &str) -> Vec<Instruction> {
    // 新ASTパーサを使い、ASTを得るだけの形に変更（命令列生成は今後追加）
    let ast = parse_program(source);
    // 今は命令列を返さず空ベクタを返す（次ステップでAST→命令列変換を追加）
    let _ = ast;
    vec![]
}
