mod chip8;
use chip8::Chip8;
use minifb::{Window, WindowOptions, Scale};
use std::fs;
use std::io;
fn main() {

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
        for _ in 0..10 {
            chip8.cycle();
        }
        chip8.tick_time();

        for i in 0..buffer.len() {
            if chip8.display[i] {
                buffer[i] = 0x00FFFF;
            } else {
                buffer[i] = 0x00000000;
            }
        }

        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}