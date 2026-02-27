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


fn parse_const_assignment(line: &str) -> ParseResult {
    line.strip_prefix("let ")
        .map(|rest| rest.split('=').map(|s| s.trim()).collect::<Vec<_>>())
        .and_then(|parts| {
            (parts.len() == 2)
                .then(|| (parts[0].to_string(), parts[1].parse::<i32>().ok()))
                .and_then(|(var, val_opt)| val_opt.map(|val| ParseResult::Assignment(var, val)))
        })
        .unwrap_or(ParseResult::None)
}

fn parse_add_assignment(line: &str, vars: &HashMap<String, i32>) -> ParseResult {
    line.strip_prefix("let ")
        .map(|rest| rest.split('=').map(|s| s.trim()).collect::<Vec<_>>())
        .and_then(|parts| {
            (parts.len() == 2)
                .then(|| {
                    let var = parts[0].to_string();
                    let rhs = parts[1];
                    rhs.contains('+')
                        .then(|| {
                            let add_parts: Vec<&str> = rhs.split('+').map(|s| s.trim()).collect();
                            (add_parts.len() == 2)
                                .then(|| (var, add_parts))
                        })
                        .flatten()
                })
                .flatten()
                .and_then(|(var, add_parts)| {
                    match (vars.get(add_parts[0]), vars.get(add_parts[1])) {
                        (Some(a), Some(b)) => Some(ParseResult::Assignment(var, a + b)),
                        _ => None,
                    }
                })
        })
        .unwrap_or(ParseResult::None)
}

fn parse_logshow(line: &str, vars: &HashMap<String, i32>) -> ParseResult {
    line.strip_prefix("logShow ")
        .map(str::trim)
        .and_then(|var| vars.get(var).map(|val| ParseResult::Instructions(vec![Instruction::Push(*val), Instruction::Print])))
        .unwrap_or(ParseResult::None)
}

fn parse_line(line: &str, vars: &HashMap<String, i32>) -> ParseResult {
    let line = line.trim();
    let parsers: [Box<dyn Fn(&str, &HashMap<String, i32>) -> ParseResult>; 3] = [
        Box::new(|l, _| parse_const_assignment(l)),
        Box::new(|l, v| parse_add_assignment(l, v)),
        Box::new(|l, v| parse_logshow(l, v)),
    ];
    parsers.iter()
        .map(|f| f(line, vars))
        .find(|res| !matches!(res, ParseResult::None))
        .unwrap_or(ParseResult::None)
}

pub fn parse_simple_purs(source: &str) -> Vec<Instruction> {
    let mut env = ParserEnv::default();
    source
        .lines()
        .flat_map(|line| {
            match parse_line(line, &env.vars) {
                ParseResult::Assignment(k, v) => {
                    env.vars.insert(k, v);
                    vec![]
                }
                ParseResult::Instructions(instrs) => instrs,
                ParseResult::None => vec![],
            }
        })
        .collect()
}
