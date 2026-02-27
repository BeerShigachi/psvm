

mod vm;
mod parser;
use vm::VM;
use parser::parse_simple_purs;

fn main() {
    let mut vm = VM::new();
    let source = r#"
let x = 2
let y = 3
let z = x + y
print z
"#;
    let instructions = parse_simple_purs(source);
    vm.run(&instructions);
}
