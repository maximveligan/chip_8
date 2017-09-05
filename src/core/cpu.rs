mod cpu {

    struct ProgramCounter(u16); 

    impl ProgramCounter {
        fn updateCounter(&mut self){
           self.0+=2; 
        }
    }

    pub struct Registers {
        pub programCounter: ProgramCounter,
        delay: u8,
        sound: u8,
        flag: u8,
        pub stackPointer: u8,
        iRegister: u16,
        pub vRegisters: [u8, 15],
    }

    impl Registers {
        fn initializeRegisters(&mut self) {
            let chip8Adrr = 0x200;
            let (mut self.delay, mut self.sound, mut self.flag, mut self.iRegister, mut self.stackPointer) = (0u8, 0u8, 0u8, 0u16, 0u8);
            let mut self.vRegisters: [u8; 15] = [0; 15];
            let self.programCounter = ProgramCounter(chip8Adrr);
        }
    }
}
