mod chip8;
use chip8::Chip8;
use std::io;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Enter ROM file name:");

    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("failed to read input");
    let path = path.trim();

    let rom = fs::read(path).expect("failed to read ROM");
    println!("Loaded {} bytes", rom.len());

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);

    loop {
        for _ in 0..10 {
            chip8.cycle();
        }
        chip8.tick_time();

        draw_terminal(&chip8);
        thread::sleep(Duration::from_millis(16));
    }
}


fn draw_terminal(chip8: &Chip8) {
    print!("\x1B[2J\x1B[H");
    for row in 0..32 {
        for col in 0..64 {
            let on = chip8.display[row * 64 + col];
            print!("{}", if on { "█" } else { " " });
        }
        println!();
    }
}
