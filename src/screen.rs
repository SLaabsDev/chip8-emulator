pub struct Screen {
    pixels: [u8; (ROW_WIDTH as usize) * (ROW_HEIGHT as usize)],
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
        for pixel in 0..((ROW_WIDTH as usize) * (ROW_HEIGHT as usize)) {
            self.pixels[pixel] = 0;
        }
    }
}

const ROW_WIDTH: u8 = 64;
const ROW_HEIGHT: u8 = 32;
