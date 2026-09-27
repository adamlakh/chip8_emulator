use rand::Rng;


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
        //from cogwork's tech reference
        let nnn = (opcode & 0x0FFF);
        let n = opcode & 0x000F;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let kk = (opcode & 0x00FF);


        let top = (opcode & 0xF000);

        match top{
            0x0000 => {
                match kk {
                    0x00E0 => {
                        // 00E0: CLS
                        self.display = [false; 64 * 32];
                    }
                    0x00EE => {
                        // 00EE: RET
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }
                    _ => {
                        // SYS addr
                        // 0x0NNN: legacy
                    }
                }
            }
            0x1000 => {
                // 1nnn: JP addr
                self.pc = nnn;
            }
            0x2000 => {
                // 2nnn: CALL addr
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            }
            0x3000 => {
                // 3xkk: SE Vx, byte
                if self.v[x as usize] == kk as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // 4xkk: SNE Vx, byte
                if self.v[x as usize] != kk as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // 5xy0: SE Vx, Vy
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // 6xkk: LD Vx, byte
                self.v[x as usize] = kk as u8;
            }
            0x7000 => {
                // 7xkk: ADD Vx, byte
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk as u8);
            }
            0x8000 => {
                match n {
                    0x0 => {
                        // 8xy0: LD Vx, Vy
                        self.v[x as usize] = self.v[y as usize];
                    }
                    0x1 => {
                        // 8xy1: OR Vx, Vy
                        self.v[x as usize] |= self.v[y as usize];
                    }
                    0x2 => {
                        // 8xy2: AND Vx, Vy
                        self.v[x as usize] &= self.v[y as usize];
                    }
                    0x3 => {
                        // 8xy3: XOR Vx, Vy
                        self.v[x as usize] ^= self.v[y as usize];
                    }
                    0x4 => {
                        // 8xy4: ADD Vx, Vy
                        let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                        self.v[x as usize] = result;
                        self.v[0xF] = overflow as u8;
                    }
                    0x5 => {
                        // 8xy5: SUB Vx, Vy
                        let (result, borrow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                        self.v[x as usize] = result;
                        self.v[0xF] = !borrow as u8;
                    }
                    0x6 => {
                        // 8xy6: SHR Vx {, Vy}
                        let lsb = self.v[x as usize] & 0x1;
                        self.v[x as usize] >>= 1;
                        self.v[0xF] = lsb;
                    }
                    0x7 => {
                        // 8XY7: SUBN Vx, Vy
                        let (result, borrow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                        self.v[x as usize] = result;
                        self.v[0xF] = !borrow as u8;
                    }
                    0xE => {
                        // 8XYE: SHL Vx {, Vy}
                        let msb = (self.v[x as usize] & 0x80) >> 7;
                        self.v[x as usize] <<= 1;
                        self.v[0xF] = msb;
                    }
                }
            }
            0x9000 => {
                // 9xy0: SNE Vx, Vy
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                // Annn: LD I, addr
                self.i = nnn;
            }
            0xB000 => {
                //Bnnn: JP V0, addr
                let value = nnn + (self.v[0x0] as u16);
                self.pc = value;
            }
            0xC000 => {
                // Cxkk: RND Vx, byte
                let random_byte = rand::thread_rng().gen::<u8>();
                self.v[x as usize] = random_byte & kk;
            }
        }
        
        //todo the match pattern
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        self.pc += 2;
        self.execute(opcode);
    }
}