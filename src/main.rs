use cpu::Cpu;

mod cpu;

fn main() {
    // initialize graphics and keyboard input
    
    let mut chip8 = Cpu::new();

    // load game
    
    loop {
        chip8.cycle();

        // update graphics
        
        // set the keys
    }
}
