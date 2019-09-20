#[derive(Debug, Clone, Copy)]
pub struct Keyboard {
    pub key_buffer: [bool; 0xF + 1],
    pub wait_press: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_buffer: [false; 0xF + 1],
            wait_press: None,
        }
    }

    pub fn press_key(&mut self, key: usize, vreg: &mut [u8; 16]) {
        self.key_buffer[key] = true;
        if self.wait_press != None {
            vreg[self.wait_press.unwrap() as usize] = key as u8;
            self.wait_press = None;
        }
    }

    pub fn release_key(&mut self, key: usize) {
        self.key_buffer[key] = false;
    }
}
