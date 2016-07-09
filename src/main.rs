use cpu::Cpu;

mod cpu;

fn main() {
    // initialize graphics and keyboard input
    
    let mut chip8 = Cpu::new();

    match chip8.load("PONG".to_string()) {
        Ok(n) => start(chip8),
        Err(err) => println!("Error: {}", err),
    }

}

fn start(mut chip8: Cpu) {
    loop {
        chip8.cycle();

        if chip8.get_draw_flag() {
            // draw
        }

        chip8.set_keys();
    }
}
