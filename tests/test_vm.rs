use psvm::parser::parse_simple_purs;
use psvm::vm::{Instruction, VM};

#[test]
fn test_instruction_add() {
    let mut vm = VM::new();
    let instructions = vec![
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Print,
    ];
    vm.run(&instructions);
    assert_eq!(vm.stack, vec![]); // printでpopされるので空
}

#[test]
fn test_simple_purs_parser_and_vm() {
    let purs_code = r#"
module Test.Simple where

main :: Effect Unit
main = do
    let x = 2
    let y = 3
    let z = x + y
    logShow z
"#;
    let instructions = parse_simple_purs(purs_code);
    let mut vm = psvm::vm::VM::new();
    vm.run(&instructions);
    assert_eq!(vm.stack, vec![]); // printでpopされるので空
}
