// The Cpu struct represents the state of the cpu for the chip-8 emulation including
// memory, registers, and graphics
pub struct Cpu {
	opcode: u16,
    
    memory: [u8; 4096],

    register: [u8; 16],
    address_register: u16,
    program_counter: u16,
    
    graphics: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u16,

    key: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu { opcode: 0, memory: [0; 4096], register: [0; 16], address_register: 0, 
                            program_counter: 0x200, graphics: [0; 64 * 32], delay_timer: 0, 
                            sound_timer: 0, stack: [0; 16], stack_pointer: 0, key: [0; 16] };

        // allocate the first portion of memory to the fontset
        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        cpu
    }

    pub fn cycle(&mut self) {
        
        self.fetch_opcode();

        // execute
        // update timers
    }

    fn fetch_opcode(&mut self) {
        self.opcode = ((self.memory[self.program_counter as usize] as u16) << 8) | 
            (self.memory[(self.program_counter + 1) as usize]) as u16
    }

    fn nop() {
        // mainly for testing purposes
    }

    fn clear_screen(&mut self) {
        for i in 0..(64 * 32) {
            self.graphics[i] = 0;
        }

        self.program_counter += 2;
    }

    fn cpu_return(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];

        self.program_counter += 2;
    }

    fn jump(&mut self) {
        self.program_counter = self.opcode & 0x0FFF;
    }

    fn call(&mut self) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;

        self.program_counter = self.opcode & 0x0FFF;
    }

    // 3XNN: Skip next instruction if VX == NN
    fn skip_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        if self.register[x as usize] == val as u8 {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }
    
    // 4XNN: Skip next instruction if VX != NN
    fn skip_not_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        if self.register[x as usize] != val as u8 {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // 5XY0: Skip next instruction if VX == VY
    fn skip_regs_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let y = self.opcode & 0x00F0;

        if self.register[x as usize] == self.register[y as usize] {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // 6XNN: Set VX = NN
    fn set_vx_num(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        self.register[x as usize] = val as u8;

        self.program_counter += 2;
    }

    // 7XNN: Add NN to VX
    fn add_vx_num(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        self.register[x as usize] += val as u8;

        self.program_counter += 2;
    }
    
    // Opcode 0x8000 instructions
    fn opcode_8(&mut self) {
        let x = self.opcode & 0x0F00;
        let y = self.opcode & 0x00F0;

        // use lookup table to call the appropriate instruction passing x and y as usize
    
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

//static OPTABLE: [fn(); 16] = [
//    nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop,
//    nullop, nullop,
//];
