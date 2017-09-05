pub enum Opcodes {



}

fn fetchOpcode(pc: ProgramCounter) -> () {
    let leftByte = ram[pc.0];
    let rightByte = ram[pc.0 + 1];
    let opcode = 
}
