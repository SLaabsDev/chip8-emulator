extern crate rand;

use std::env;
use std::io::prelude::*;
use std::fs::File;

use self::rand::Rng;

use keys::Keys;
use screen::Screen;

// The Cpu struct represents the state of the cpu for the chip-8 emulation including
// memory, registers, and graphics
pub struct Cpu {
    opcode: u16,
   
    pub draw_flag: bool,

    memory: [u8; 4096],

    register: [u8; 16],
    address_register: u16,
    program_counter: u16,
    

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u16,

    pub graphics: Screen,
    pub keypad: Keys,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu { opcode: 0, draw_flag: true, memory: [0; 4096], register: [0; 16], address_register: 0, 
                            program_counter: 0x200, graphics: Screen::new(), delay_timer: 0, 
                            sound_timer: 0, stack: [0; 16], stack_pointer: 0, keypad: Keys::new() };

        // allocate the first portion of memory to the fontset
        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        cpu
    }

    pub fn load(&mut self, rom: String) -> Result<i32, String> {
        let mut rom_path = env::current_dir().unwrap();
       
        rom_path.push("rom");
        rom_path.push(rom);

        println!("ROM: {}", rom_path.display());

        let rom_file = try!(File::open(rom_path).map_err(|e| e.to_string()));

        let mut address = self.program_counter;
        for byte in rom_file.bytes() {
            self.memory[address as usize] = byte.unwrap();
            address += 1;
        }
        
        Ok(1)
    }

    pub fn cycle(&mut self) { 
        self.fetch_opcode();

        self.execute();

        self.step_timers();
    }

    fn fetch_opcode(&mut self) {
        self.opcode = ((self.memory[self.program_counter as usize] as u16) << 8) | 
            (self.memory[(self.program_counter + 1) as usize]) as u16
    }

    fn execute(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => self.opcode_0(),
            0x1000 => self.jump(),
            0x2000 => self.call(),
            0x3000 => self.skip_equal(),
            0x4000 => self.skip_not_equal(),
            0x5000 => self.skip_regs_equal(),
            0x6000 => self.set_vx_num(),
            0x7000 => self.add_vx_num(),
            0x8000 => self.opcode_8(),
            0x9000 => self.skip_regs_not_equal(),
            0xA000 => self.set_adr_reg(),
            0xB000 => self.jump_add(),
            0xC000 => self.rand_op(),
            0xD000 => self.draw(),
            0xE000 => self.skip_key_press(),
            0xF000 => self.opcode_f(),
            _      => println!("Opcode not unimplemented: {:X}", self.opcode),
        }
    }

    fn step_timers(&mut self) {
        // TODO: Step the timers at 60Hz
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: Implement a beep sound
                //println!("SoundTimer: BEEP");
            }

            self.sound_timer -= 1;
        }
    }

    // 0x0000 instructions
    fn opcode_0(&mut self) {
        match self.opcode & 0x00FF {
            0x00E0 => self.clear_screen(),
            0x00EE => self.cpu_return(),
            _ => println!("Opcode not unimplemented: {:X}", self.opcode),
        }
    }

    // 0x00E0
    fn clear_screen(&mut self) {
        self.graphics.clear();
        self.draw_flag = true;
        
        self.program_counter += 2;
    }

    // 0x00EE
    fn cpu_return(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];

        self.program_counter += 2;
    }

    // 0x1NNN
    fn jump(&mut self) {
        self.program_counter = self.opcode & 0x0FFF;
    }

    // 0x2NNN
    fn call(&mut self) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;

        self.program_counter = self.opcode & 0x0FFF;
    }

    // 3XNN: Skip next instruction if VX == NN
    fn skip_equal(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let val = self.opcode & 0x00FF;

        if self.register[x as usize] == val as u8 {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }
    
    // 4XNN: Skip next instruction if VX != NN
    fn skip_not_equal(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let val = self.opcode & 0x00FF;

        if self.register[x as usize] != val as u8 {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // 5XY0: Skip next instruction if VX == VY
    fn skip_regs_equal(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        if self.register[x as usize] == self.register[y as usize] {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // 6XNN: Set VX = NN
    fn set_vx_num(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let val = self.opcode & 0x00FF;

        self.register[x as usize] = val as u8;

        self.program_counter += 2;
    }

    // 7XNN: Add NN to VX
    fn add_vx_num(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let val = self.opcode & 0x00FF;

        let sum: u16 = self.register[x as usize] as u16 + val;

        self.register[x as usize] = (sum & 0x00FF) as u8;

        self.program_counter += 2;
    }
    
    // Opcode 0x8000 instructions
    fn opcode_8(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        match self.opcode & 0x000F {
            0x0 => self.set_vx_vy(x as usize, y as usize),
            0x1 => self.set_vx_or(x as usize, y as usize),
            0x2 => self.set_vx_and(x as usize, y as usize),
            0x3 => self.set_vx_xor(x as usize, y as usize),
            0x4 => self.add_vx_vy(x as usize, y as usize),
            0x5 => self.sub_vx_vy(x as usize, y as usize),
            0x6 => self.shr_vx(x as usize),
            0x7 => self.sub_vy_vx(x as usize, y as usize),
            0xE => self.shl_vx(x as usize),
            _   => println!("Opcode not unimplemented: {:X}", self.opcode),
        }

        self.program_counter += 2;
    }

    // 8XY0: Set VX to VY
    fn set_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[y];
    }

    // 8XY1: Set VX = VX | VY
    fn set_vx_or(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[x] | self.register[y];
    }

    // 8XY2: Set VX = VX & VY
    fn set_vx_and(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[x] & self.register[y];
    }

    // 8XY3: Set VX = VX ^ VY
    fn set_vx_xor(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[x] & self.register[y];
    }

    // 8XY4: VX += VY with carry flag
    fn add_vx_vy(&mut self, x: usize, y: usize) {
        let sum: u16 = self.register[x] as u16 + self.register[y] as u16;
        
        if sum > 255 {
            self.register[0xF] = 1;
        } 
        else {
            self.register[0xF] = 0;
        }

        // dropping higher bits with the carry
        self.register[x] = (sum & 0x00FF) as u8;
    }

    // 8XY5: VX -= VY with borrow flag
    fn sub_vx_vy(&mut self, x: usize, y: usize) { 
        if self.register[x] < self.register[y] {
            self.register[0xF] = 0;
            self.register[x] = 255 - (self.register[y] - self.register[x]) + 1;
        }
        else {
            self.register[0xF] = 1;
            self.register[x] -= self.register[y];
        }
    }

    // 8XY6: Set VF to LSB of VX then shift VX right
    fn shr_vx(&mut self, x: usize) {
        self.register[0xF] = self.register[x] & 1;
        self.register[x] = self.register[x] >> 1;
    }

    // 8XY7: Set VX = VY - VX and set borrow flag
    fn sub_vy_vx(&mut self, x: usize, y: usize) {
        if self.register[y] < self.register[x] {
            self.register[0xF] = 0;
        }
        else {
            self.register[0xF] = 1;
        }

        self.register[x] = self.register[y] - self.register[x];
    }

    // 8XYE: Set flag to MSB then shift VX left
    fn shl_vx(&mut self, x: usize) {
        self.register[0xF] = self.register[x] & 0x80;
        self.register[x] = self.register[x] << 1;
    }

    // 9XY0: Skip the instruction if VX != VY
    fn skip_regs_not_equal(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;

        if self.register[x as usize] != self.register[y as usize] {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // ANNN: Set address register
    fn set_adr_reg(&mut self) {
        self.address_register = self.opcode & 0x0FFF;

        self.program_counter += 2;
    }

    // BNNN: Jump to address NNN+V0
    fn jump_add(&mut self) {
        self.program_counter = (self.opcode & 0x0FFF) + self.register[0] as u16;
    }

    // CXNN: VX = bitwise and operation on random num and given num
    fn rand_op(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let value = self.opcode & 0x00FF;
        
        let mut rng = rand::thread_rng();

        self.register[x as usize] = (value as u8) & rng.gen::<u8>();

        self.program_counter += 2;
    }

    // DXYN: Draw sprite at position X,Y with N rows
    fn draw(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let rows = self.opcode & 0x000F; 

        self.register[0xF] = self.graphics.draw(self.register[x as usize] as usize, self.register[y as usize] as usize, &self.memory[(self.address_register as usize)..(self.address_register as usize + rows as usize)]);
    
        self.draw_flag = if self.register[0xF] != 0 { false } else { true };
        self.program_counter += 2;
    }

    // 0xEX9E and EXA1 skip instruction of key in VX if it is pressed/not pressed depending on
    // opcode
    fn skip_key_press(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let key = self.register[x as usize];

        match self.opcode & 0x00FF {
            0x9E => {
                if self.keypad.is_down(key as usize) {
                    self.program_counter += 2;
                }
            }, 
            
            0xA1 => {         	
                if !self.keypad.is_down(key as usize) {
                    self.program_counter += 2;
                }
            },

            _ => println!("Opcode not unimplemented: {:X}", self.opcode),
        }

        self.program_counter += 2;
    }

    // Opcode 0xF000 instructions
    fn opcode_f(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        match self.opcode & 0x00FF {
            0x07 => self.set_vx_delay(x as usize),
            0x0A => self.set_vx_key(x as usize),
            0x15 => self.set_delay_vx(x as usize),
            0x18 => self.set_sound_vx(x as usize),
            0x1E => self.add_adr_reg(x as usize),
            0x29 => self.set_adr_char(x as usize),
            0x33 => self.vx_to_bcd(x as usize),
            0x55 => self.store_regs(x as usize),
            0x65 => self.read_regs(x as usize),
            _ => println!("Opcode not unimplemented: {:X}", self.opcode),
        }
    }

    // FX07: VX = delay_timer
    fn set_vx_delay(&mut self, x: usize) {
        self.register[x] = self.delay_timer;
        self.program_counter += 2;
    }

    // FX0A: Store key press in VX
    fn set_vx_key(&mut self, x: usize) {
        let mut pressed = false;

        for i in 0..16 {
            if self.keypad.is_down(i) {
                self.register[x] = i as u8;
                pressed = true;
            }
        }

        // stay on this instruction until a key is pressed down
        if pressed {
            self.program_counter += 2;
        }
    }

    // FX15: Set delay_timer = VX
    fn set_delay_vx(&mut self, x: usize) {
        self.delay_timer = self.register[x];
        self.program_counter += 2;
    }

    // FX18: Set sound_timer = VX
    fn set_sound_vx(&mut self, x: usize) {
        self.sound_timer = self.register[x];
        self.program_counter += 2;
    }

    // FX1E: Add VX to address pointer
    fn add_adr_reg(&mut self, x: usize) {
        self.address_register += self.register[x] as u16;
        self.program_counter += 2;
    }

    // FX29: Set address pointer to memory address of the character in VX
    fn set_adr_char(&mut self, x: usize) {
        self.address_register = (self.register[x] as u16) * 5;

        self.program_counter += 2;
    }

    // FX33: Save register X at address pointer in 3 bytes as a binary-coded decimal
    fn vx_to_bcd(&mut self, x: usize) {
        self.memory[self.address_register as usize] = self.register[x] / 100; // digit 1
        self.memory[(self.address_register as usize) + 1] = (self.register[x] / 10) % 10; // digit 2
        self.memory[(self.address_register as usize) + 2] = (self.register[x] % 100) % 10; // digit 3

        self.program_counter += 2;
    }

    // FX55: Store registers 0..X inclusive into memory starting at address pointer
    fn store_regs(&mut self, x: usize) {
        for reg in 0..(x+1) {
            self.memory[self.address_register as usize + reg] = self.register[reg];
        }

        self.program_counter += 2;
    }

    // FX65: Read registers 0..X inclusive from memory starting at address pointer
    fn read_regs(&mut self, x: usize) {
        for reg in 0..(x+1) {
            self.register[reg] = self.memory[self.address_register as usize + reg];
        }

        self.program_counter += 2;
    }
}

// Each character in the font set is 5 characters hide and 4 pixels wide
// so each entry represents one row in a character
static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
