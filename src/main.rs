mod chip8;
use chip8::Chip8;
use minifb::{Window, WindowOptions, Scale, Key};
use std::fs;
use std::io;

fn main() {
    const KEYMAP: [Key; 16] = [
        Key::X,    // 0
        Key::Key1, // 1
        Key::Key2, // 2
        Key::Key3, // 3
        Key::Q,    // 4
        Key::W,    // 5
        Key::E,    // 6
        Key::A,    // 7
        Key::S,    // 8
        Key::D,    // 9
        Key::Z,    // A
        Key::C,    // B
        Key::Key4, // C
        Key::R,    // D
        Key::F,    // E
        Key::V,    // F
    ];

    println!("Enter ROM file name:");

    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("failed to read input");
    let path = path.trim();

    let rom = fs::read(path).expect("failed to read ROM");

    println!("Loaded {} bytes", rom.len());

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);

    let mut window = Window::new(
        "CHIP-8 test",
        64,
        32,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    ).expect("couldn't to create window");

    let mut buffer = vec![0u32; 64*32];

    for i in 0..buffer.len() {
        if chip8.display[i] {
            buffer[i] = 0x00FFFFFF;
        } else {
            buffer[i] = 0x00000000;
        }
    }

    while window.is_open() {
        for key_index in 0..KEYMAP.len() {
            chip8.keypad[key_index] = window.is_key_down(KEYMAP[key_index]);
        }

        for _ in 0..7 {
            chip8.cycle();
        }
        chip8.tick_time();

        for i in 0..buffer.len() {
            if chip8.display[i] {
                buffer[i] = 0x0000FFFF;
            } else {
                buffer[i] = 0x00000000;
            }
        }

        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}