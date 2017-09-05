struct Interpreter {}

impl CPU for Interpreter {
    fn emulateCycle(opcode: u16) -> () {
        match opcode {
            0x00E0 => println!("SYS addr"),
            0x00EE => println!("RET"),
            _ => opcodesWOperands(opcode),
        }
    }
}

fn matchOpcode(opcode: u16) -> () {
//  First check to see if opcode has operands
    match opcode {
        0x00E0 => //opcode
        0x00EE => //opcode
        _ => match (opcode & 0xF000) {

            //  If we get here, it means opcode has an argument

            0xF000 => match (opcode & 0x0060) {
                0x0060 => //ld x
                0x0050 => //ld i vx
                0x0030 => //ld b vx
                0x0020 => //ld f vx
                0x0010 => match (opcode & 0x000e) {
                    0x000E => //opcode
                    0x0008 => //opcode
                    0x0005 => //opcode
                0x0000 => match (opcode & 0x000A) {
                    0x000A => //opcode
                    0x0007 => //opcode
                    0x0001 => //opcode

            0xE000 => println!("Opcode starts with an E"),
            0xD000 => //todo: expand
            0xC000 => //todo: expand
            0xB000 => //todo: expand
            0xA000 => //todo: expand
            0x9000 => //todo: expand
            0x8000 => //todo: expand
            0x7000 => //todo: expand
            0x8000 => //todo: expand
            0x6000 => //todo: expand
            0x5000 => //todo: expand
            0x4000 => //todo: expand
            0x3000 => //todo: expand
            0x2000 => //todo: expand
            0x1000 => //todo: expand
            0x0000 => //todo: expand

