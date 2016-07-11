extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::{Rect};

use cpu::Cpu;

mod cpu;
mod keys;
mod screen;

fn main() {
    // initialize graphics and keyboard input
    
    let mut chip8 = Cpu::new();

    match chip8.load("PONG".to_string()) {
        Ok(..) => start(chip8),
        Err(err) => println!("Error: {}", err),
    }

}

fn start(mut chip8: Cpu) {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    // make window
    let window = match video_subsystem.window("Chip8 Emulator", 64*20, 32*20).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("Failed to create window: {}", err)
    };

    // make renderer
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("Failed to create renderer: {}", err)
    };

    let mut events = sdl_context.event_pump().unwrap();
    
    'cpu_cycle: loop {
        chip8.cycle();

        if chip8.draw_flag {

            chip8.graphics.show();

            let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

            let test_rect = Rect::new(0, 0, 128, 128);
            let _ = renderer.draw_rect(test_rect);
            let _ = renderer.present();

            chip8.draw_flag = false;
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit {..}    => break 'cpu_cycle,
                Event::KeyDown { keycode, ..} => chip8.keypad.set_keys(keycode, false),
                Event::KeyUp { keycode, ..}   => chip8.keypad.set_keys(keycode, true),
                _ => {},
            }
        }

    }
}
