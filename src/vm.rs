
pub struct VM {
    pub stack: Vec<i32>,
}

pub enum Instruction {
    Push(i32),
    Add,
    Print,
}

impl VM {
    pub fn new() -> Self {
        VM { stack: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    pub fn add(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(a + b);
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.stack);
    }

    pub fn run(&mut self, instructions: &[Instruction]) {
        for instr in instructions {
            match instr {
                Instruction::Push(val) => self.push(*val),
                Instruction::Add => self.add(),
                Instruction::Print => self.print(),
            }
        }
    }
}
