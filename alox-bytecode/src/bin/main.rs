use alox_bytecode::{chunk::Chunk, opcodes::Op, vm::Vm};

fn main() {
    let mut chunk = Chunk::init();
    chunk.write_constant(1.2, 123);
    chunk.write(Op::Negate.u8(), 123);
    chunk.write(Op::Return.u8(), 123);
    let mut vm = Vm::new(chunk);
    if let Err(err) = vm.interpret() {
        eprintln!("{:?}", err);
    }

    //chunk.disassemble("test chunk")
}
