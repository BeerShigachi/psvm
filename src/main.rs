
mod vm;
use vm::{VM, Instruction};

fn main() {
    let mut vm = VM::new();
    let instructions = vec![
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Print,
    ];
    vm.run(&instructions);
}
