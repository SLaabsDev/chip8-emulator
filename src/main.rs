extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use cpu::Cpu;

mod cpu;
mod keys;

fn main() {
    // initialize graphics and keyboard input
    
    let mut chip8 = Cpu::new();

    match chip8.load("PONG".to_string()) {
        Ok(n) => start(chip8),
        Err(err) => println!("Error: {}", err),
    }

}

fn start(mut chip8: Cpu) {
    let sdl_context = sdl2::init().unwrap();

    let mut events = sdl_context.event_pump().unwrap();
    
    'running: loop {
        chip8.cycle();

        if chip8.get_draw_flag() {
            // draw
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit {..}    => break 'running,
                Event::KeyDown { keycode, ..} => chip8.keypad.set_keys(keycode, false),
                Event::KeyUp { keycode, ..}   => chip8.keypad.set_keys(keycode, true),
                _ => {},
            }
        }

    }
}
