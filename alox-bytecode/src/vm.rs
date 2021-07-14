use crate::{
    chunk::{Chunk, Value},
    opcodes::Op,
};

pub type InterpreterResult = Result<(), InterpreterError>;
pub struct Vm {
    chunk: Chunk,
    ip: usize,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm { chunk, ip: 0 }
    }
    pub fn interpret(&mut self) -> InterpreterResult {
        self.run()
    }

    fn run(&mut self) -> InterpreterResult {
        Ok(loop {
            if self.ip >= self.chunk.code.len() {
                break;
            }
            let next_byte = self.next_byte();
            let instruction = Op::from_u8(next_byte);
            dbg!(instruction);
            match instruction {
                Op::Return => return Ok(()),
                Op::Constant => {
                    let index = self.next_byte();
                    let constant = self.read_constant(index);
                    println!("{}", constant);
                }
                Op::ConstantLong => {}
            }
        })
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&self, index: u8) -> Value {
        self.chunk.constants[index as usize]
    }
}

#[derive(Debug)]
pub enum InterpreterError {
    CompileError,
    RuntimeError,
    NoInstructions,
    UnknownInstruction,
}
