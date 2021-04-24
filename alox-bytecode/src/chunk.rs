use crate::opcodes::Op;
use std::convert::TryFrom;
use std::usize;
pub type Value = f64;
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn init() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }
    pub fn write(&mut self, byte: u8, line: usize) {
        self.lines.push(line);
        self.code.push(byte);
    }

    pub fn disassemble(&mut self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        loop {
            if offset >= self.code.len() { break; }

            offset = self.disassemble_instruction(offset);
        }

    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("    | ");
        } else {
            print!("  {} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        let opcode = Op::try_from(instruction);

        if let Ok(op) = opcode {
            match op {
                Op::Constant => self.print_constant_instruction(op, offset),
                _default =>  {
                    println!("{:?}", op);
                    return offset + 1;
                }
            }
        } else {
            println!("Unknown opcode {}", instruction);
            return offset + 1;
        }
    }

    fn print_constant_instruction(&self, op: Op, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        let value = self.constants[constant as usize];
        println!("{:?} \t{} '{}'", op, offset, value);
        offset + 2
    }
}
