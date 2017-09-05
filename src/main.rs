mod cpu;
mod ram;

fn main() {
    let interpreter: Interpreter = 
    ram.initialize();
    cpu.initialize();
    rom.load("path_to_rom")
    startEmulator();
}

fn startEmulator() -> () {
    loop {
        emulateCPUCycle();
        updateGPU();
    }
}
