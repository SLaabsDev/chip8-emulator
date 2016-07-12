extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::{Rect};

use cpu::Cpu;
use screen::ROW_WIDTH;
use screen::ROW_HEIGHT;
use screen::SCALE;

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
    let window = match video_subsystem.window("Chip8 Emulator", ROW_WIDTH*SCALE, ROW_HEIGHT*SCALE).position_centered().opengl().build() {
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
            
            //chip8.graphics.show();
            for row in 0..ROW_HEIGHT {
                for column in 0..ROW_WIDTH {
                    let pos_x = column * SCALE;
                    let pos_y = row * SCALE;

                    let pixel_rect = Rect::new(pos_x as i32, pos_y as i32, SCALE, SCALE);
                    
                    let color = if chip8.graphics.get_pixel(column as usize, row as usize) != 0 { 255 } else { 0 };
                    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(color, color, color));
                    
                    let _ = renderer.fill_rect(pixel_rect);
                }
            }
            
            let _ = renderer.present();

            chip8.draw_flag = false;
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit {..}    => break 'cpu_cycle,
                Event::KeyDown { keycode, ..} => chip8.keypad.set_keys(keycode, true),
                Event::KeyUp { keycode, ..}   => chip8.keypad.set_keys(keycode, false),
                _ => {},
            }
        }

    }
}
