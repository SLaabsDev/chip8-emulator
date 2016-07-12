//use sdl2::renderer;
//use sdl2::rect;

pub struct Screen {
    pub pixels: [u8; (ROW_WIDTH as usize) * (ROW_HEIGHT as usize)],
    //renderer: 
}

impl Screen {
    pub fn new() -> Screen {
        Screen { pixels: [0; (ROW_WIDTH as usize) * (ROW_HEIGHT as usize)] }
    }

    // Returns a value for VF to be set to indicate a bit being cleared
    pub fn draw(&mut self, x: usize, y: usize, data: &[u8]) -> u8 {
        let mut flag: u8 = 0;
        
        for row in 0..data.len() {
            // TODO: Make pixels wrap around screen rather than go out of bounds
            let byte: u8 = data[row];

            for column in 0..8 {
                if (byte & (0x80 >> column)) != 0 {
                    let index: usize = ((y + row) * ROW_WIDTH as usize) + x + column as usize;
                    
                    if self.pixels[index] == 1 {
                        flag = 1;
                    }

                    self.pixels[index] ^= 1;
                }
            }
        }

        flag
    }

    pub fn clear(&mut self) {
        self.pixels = [0; (ROW_WIDTH as usize) * (ROW_HEIGHT as usize)];
    }
    
    pub fn get_pixel(& self, x: usize, y: usize) -> u8 {
    	self.pixels[(y * ROW_WIDTH as usize) + x]
    }
}

pub const ROW_WIDTH: u32 = 64;
pub const ROW_HEIGHT: u32 = 32;
pub const SCALE: u32 = 20;

