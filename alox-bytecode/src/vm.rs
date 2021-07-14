use crate::{chunk::{Chunk, Value}, opcodes::Op};

const STACK_MAX: usize = 256;

pub type InterpreterResult = Result<(), InterpreterError>;
pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn interpret(&mut self) -> InterpreterResult {
        self.run()
    }

    fn run(&mut self) -> InterpreterResult {
        Ok(loop {
            if self.ip >= self.chunk.code.len() {
                break;
            }
            #[cfg(debug_assertions)]
            println!("{:?}", &self.stack);
            let next_byte = self.next_byte();
            let instruction = Op::from_u8(next_byte);
            #[cfg(debug_assertions)]
            self.chunk.disassemble_instruction(self.ip - 1);
            match instruction {
                Op::Return => {
                    println!("{}", self.pop())
                }
                Op::Constant | Op::ConstantLong => {
                    let index = self.next_byte();
                    let constant = self.read_constant(index);
                    self.push(constant);
                }
                Op::Negate => {
                    let operand = self.pop();
                    self.push(-operand);
                }
                _ => {}
            }
        })
    }

    #[inline]
    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack Underflow!")
    }

    #[inline]
    fn push(&mut self, value: Value) {
        self.stack.push(value)
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
