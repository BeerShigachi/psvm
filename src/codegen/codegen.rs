use crate::ast::ast::*;
use crate::vm::Instruction;

pub fn expr_to_instructions(expr: &Expr) -> Vec<Instruction> {
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

pub fn stmt_to_instructions(stmt: &Stmt) -> Vec<Instruction> {
    match stmt {
        Stmt::Let(_name, expr) => expr_to_instructions(expr),
        Stmt::Print(expr) => {
            let mut code = expr_to_instructions(expr);
            code.push(Instruction::Print);
            code
        }
        Stmt::ExprStmt(expr) => expr_to_instructions(expr),
    }
}

pub fn program_to_instructions(prog: &Program) -> Vec<Instruction> {
    prog.0.iter().flat_map(stmt_to_instructions).collect()
}
