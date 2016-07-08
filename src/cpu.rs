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
        self.opcode = ((self.memory[self.program_counter] as u16) << 8) | (self.memory[self.program_counter + 1] as u16);
        // decode
        // execute
        // update timers
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
