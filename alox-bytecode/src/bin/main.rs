use alox_bytecode::{chunk::Chunk, vm::Vm};

fn main() {
    let mut chunk = Chunk::init();
    chunk.write_constant(1.2, 123);
    let mut vm = Vm::new(chunk);
    if let Err(err) = vm.interpret() {
        eprintln!("{:?}", err);
    }

    //chunk.disassemble("test chunk")
}
