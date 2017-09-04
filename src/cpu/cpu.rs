//Initialize Registers, seperate flag from other registers to stop accidental
//binding of flags
//
//Clear all registers on initialization
//
let mut vRegisters: [u8; 15] = [0; 15];
let (mut delay, mut sound, mut flag, mut iRegister, mut programCounter, mut stackPointer) = (0u8, 0u8, 0u8, 0u16, 0u16, 0u8);

//Bind RAM into array of mutable elements. Define memory banks as references
//to original ram structure.
let mut ram: [u8; 0xFFF] = [0; 0xFFF];
let interpreterMem = &ram[0x000, 0x1FF];
let chip8STDMem = &ram[0x200, 0x5FF];
let chipETIMem = &ram[0x600, 0xFFF];
