# chip8_emulator

A CHIP-8 emulator written in Rust. I made it to learn the language
and to understand how an emulator works: fetch, decode, execute.

![Pong running in the emulator](docs/rocket.png)

## Running it

You need Rust installed (https://rustup.rs).

```
git clone https://github.com/adamlakh/chip8_emulator
cd chip8_emulator
cargo run --release -- path/to/rom.ch8
```

Most ROMs aren't included. I tested with ones from <[chip8-roms](https://github.com/kripod/chip8-roms)>.

## Controls

CHIP-8 has a 16-key hex keypad, mapped onto the left side of the keyboard:

```
1 2 3 4        1 2 3 C
Q W E R   ->   4 5 6 D
A S D F        7 8 9 E
Z X C V        A 0 B F
```

## What works

- All 35 opcodes (or: all except X, Y)
- Sound timer / beep (yes/no)
- Tested with: Pong, the Corax+ test ROM, and much more

## What doesn't (yet)

- No SUPER-CHIP support
- Some games run too fast or too slow

## Resources

- [Cowgod's](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) CHIP-8 technical reference
- [chip8-roms](https://github.com/kripod/chip8-roms): ROMs
