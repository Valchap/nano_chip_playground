#![feature(bigint_helper_methods)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]

use std::io;

use crate::nano_chip_emulator::NanoChipEmulator;

mod nano_chip_emulator;

fn main() {
    if let Some(rom_file) = std::env::args().nth(1) {
        match std::fs::read(rom_file) {
            Ok(input_bytes) => {
                if input_bytes.len() <= 512 {
                    let mut rom = [0u16; 256];

                    for i in 0..input_bytes.len() / 2 {
                        rom[i] = (input_bytes[i * 2] as u16) << 8 | input_bytes[i * 2 + 1] as u16
                    }

                    let mut emulator = NanoChipEmulator::new(&rom);

                    let stdin = io::stdin();

                    loop {
                        emulator.print_status();
                        emulator.tick();
                        stdin.read_line(&mut String::new()).unwrap();
                    }
                } else {
                    println!("Error rom file size must 512 bytes");
                }
            }
            Err(read_error) => {
                println!("Error can't read input file : {read_error}");
            }
        }
    } else {
        println!("Error, a file is needed as parameter");
    }
}
