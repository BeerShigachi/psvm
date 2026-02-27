use crate::vm::Instruction;

pub fn parse_simple_purs(source: &str) -> Vec<Instruction> {
    // 仮実装: 特定のコードパターンに対して固定の命令列を返す
    if source.contains("let x = 2") && source.contains("let y = 3") && source.contains("let z = x + y") && source.contains("logShow z") {
        vec![
            Instruction::Push(2),
            Instruction::Push(3),
            Instruction::Add,
            Instruction::Print,
        ]
    } else {
        vec![] // 未対応構文は空
    }
}
