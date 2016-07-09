use sdl2::keyboard::Keycode;

pub struct Keys {
    key: [bool; 16],
}

impl Keys {
    pub fn new() -> Keys {
        Keys { key: [false; 16] }
    }

    pub fn is_down(& self, id: usize) -> bool {
        self.key[id]
    }

    pub fn set_keys(&mut self, option_code: Option<Keycode>, state: bool) {
        match option_code {
            None => return,
            Some(code) => {               
                match code {
                    Keycode::Num1 => self.key[0x1] = state,
                    Keycode::Num2 => self.key[0x2] = state,
                    Keycode::Num3 => self.key[0x3] = state,
                    Keycode::Num4 => self.key[0xC] = state,
                    Keycode::Q    => self.key[0x4] = state,
                    Keycode::W    => self.key[0x5] = state,
                    Keycode::E    => self.key[0x6] = state,
                    Keycode::R    => self.key[0xD] = state,
                    Keycode::A    => self.key[0x7] = state,
                    Keycode::S    => self.key[0x8] = state,
                    Keycode::D    => self.key[0x9] = state,
                    Keycode::F    => self.key[0xE] = state,
                    Keycode::Z    => self.key[0xA] = state,
                    Keycode::X    => self.key[0x0] = state,
                    Keycode::C    => self.key[0xB] = state,
                    Keycode::V    => self.key[0xF] = state,
                    _             => {},
                }
            }
        }
        
    }
}
