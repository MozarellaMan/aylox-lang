use alox_bytecode::{chunk::Chunk, opcodes::Op};

fn main() {
    let mut chunk = Chunk::init();
    let constant = chunk.add_constant(1.2);
    chunk.write(Op::Constant.u8(), 123);
    chunk.write(constant as u8, 123);
    chunk.write(Op::Return.u8(), 123);
    chunk.disassemble("test chunk")
}
