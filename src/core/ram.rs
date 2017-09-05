pub mod ram {

    pub struct Ram {
        wholeBank: [u8, 0xFFF],
        interpreterMem: &[u8],
        pub chip8STDMem: &[u8],
        chipETIMem: &[u8],
    }

    impl Ram {
        pub fn initializeRam(&mut self) {
            let mut self.wholeBank: [u8; 0xFFF] = [0; 0xFFF];
            let self.interpreterMem = wholeBank[0x000, 0x1FF];
            let self.chip8STDMem = &mut wholeBank[0x200, 0x5FF];
            let self.chipETIMem = &mut wholeBank[0x600, 0xFFF];
        }
    }
}
