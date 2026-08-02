
pub struct Chip8 {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub keypad: [bool; 16],
    pub display: [bool; 64 * 32],
}

const FONT_SET: [u8; 80] = [
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

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0u8; 4096];
        for index in 0 .. 80 {
            memory[index] = FONT_SET[index];
        }
        Chip8 {
            memory,
            v: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            keypad: [false; 16],
            display: [false; 64 * 32],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]){
        let start = 0x200;
        let end = start + rom.len();
        self.memory[start..end].copy_from_slice(rom);
    }

    pub fn fetch(&self) -> u16 {
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[(self.pc) as usize + 1];
        ((byte1 as u16) << 8) | (byte2 as u16)
    }

    pub fn execute(&mut self, opcode: u16) {
        let top = (opcode & 0xF000);
        let n = opcode & 0x000F;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let nnn = (opcode & 0x0FFF);
        let kk = (opcode & 0x00FF);

        //todo the match pattern
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        self.pc += 2;
        self.execute(opcode);
    }
}