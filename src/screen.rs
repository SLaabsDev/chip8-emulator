pub struct Screen {
    pixels: [u8; 64 * 32],
}

impl Screen {
    pub fn new() -> Screen {
        Screen { pixels: [0; 64 * 32] };
    }

    // Returns a value for VF to be set to indicate a bit being cleared
    pub fn draw(&mut self, x: usize, y: usize, height: u8) -> u8 {
        
    }
}
