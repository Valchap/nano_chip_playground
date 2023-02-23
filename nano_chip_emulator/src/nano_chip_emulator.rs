pub struct NanoChipEmulator<'a> {
    rom: &'a [u16; 256],
    ram: [u8; 256],
    accumulator: u8,
    z_flag: bool,
    c_flag: bool,
    v_flag: bool,
    n_flag: bool,
    pc: u8,
}

impl NanoChipEmulator<'_> {
    pub const fn new(rom: &[u16; 256]) -> NanoChipEmulator {
        NanoChipEmulator {
            rom,
            ram: [0; 256],
            accumulator: 0,
            z_flag: false,
            c_flag: false,
            v_flag: false,
            n_flag: false,
            pc: 0,
        }
    }

    /// Execute a single instruction
    pub fn tick(&mut self) {
        let instruction = self.rom[self.pc as usize];

        let opcode = (instruction >> 8) as u8;
        let operand = (instruction & 0xFF) as u8;

        match opcode {
            // ST
            0x01 => {
                self.ram[operand as usize] = self.accumulator;

                self.pc += 1;
            }

            // LD const
            0x02 => {
                self.accumulator = operand;

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // LD address
            0x03 => {
                self.accumulator = self.ram[operand as usize];

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // AND const
            0x04 => {
                self.accumulator &= operand;

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // AND addr
            0x05 => {
                self.accumulator &= self.ram[operand as usize];

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // OR const
            0x06 => {
                self.accumulator |= operand;

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // OR addr
            0x07 => {
                self.accumulator |= self.ram[operand as usize];

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // XOR const
            0x08 => {
                self.accumulator ^= operand;

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // XOR addr
            0x09 => {
                self.accumulator ^= self.ram[operand as usize];

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // ROL acc
            0x0A => {
                (self.accumulator, self.c_flag) = self.accumulator.overflowing_shl(1);
                self.accumulator |= if self.c_flag { 1 } else { 0 };

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // ROR acc
            0x0B => {
                (self.accumulator, self.c_flag) = self.accumulator.overflowing_shr(1);
                self.accumulator |= if self.c_flag { 0b10000000 } else { 0 };

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // ADD const
            0x0C => {
                let a_sign = self.accumulator > 0x7F;
                let b_sign = operand > 0x7F;

                (self.accumulator, self.c_flag) = self.accumulator.overflowing_add(operand);

                self.v_flag = a_sign == b_sign && a_sign != (self.accumulator > 0x7F);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // ADD addr
            0x0D => {
                let a_sign = self.accumulator > 0x7F;
                let b_sign = self.ram[operand as usize] > 0x7F;

                (self.accumulator, self.c_flag) =
                    self.accumulator.overflowing_add(self.ram[operand as usize]);

                self.v_flag = a_sign == b_sign && a_sign != (self.accumulator > 0x7F);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // ADC const
            0x0E => {
                let a_sign = self.accumulator > 0x7F;
                let b_sign = operand > 0x7F;

                (self.accumulator, self.c_flag) =
                    self.accumulator.carrying_add(operand, self.c_flag);

                self.v_flag = a_sign == b_sign && a_sign != (self.accumulator > 0x7F);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // ADC addr
            0x0F => {
                let a_sign = self.accumulator > 0x7F;
                let b_sign = self.ram[operand as usize] > 0x7F;

                (self.accumulator, self.c_flag) = self
                    .accumulator
                    .carrying_add(self.ram[operand as usize], self.c_flag);

                self.v_flag = a_sign == b_sign && a_sign != (self.accumulator > 0x7F);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // NEG acc
            0x10 => {
                self.accumulator = self.accumulator.wrapping_neg();

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // NEG const
            0x11 => {
                self.accumulator = operand.wrapping_neg();

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // NEG addr
            0x12 => {
                self.accumulator = self.ram[operand as usize].wrapping_neg();

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // INC acc
            0x13 => {
                (self.accumulator, self.c_flag) = self.accumulator.overflowing_add(1);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // INC addr
            0x14 => {
                (self.accumulator, self.c_flag) = self.ram[operand as usize].overflowing_add(1);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // DEC acc
            0x15 => {
                (self.accumulator, self.c_flag) = self.accumulator.overflowing_sub(1);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // DEC addr
            0x16 => {
                (self.accumulator, self.c_flag) = self.ram[operand as usize].overflowing_sub(1);

                self.z_flag = self.accumulator == 0;
                self.n_flag = self.accumulator > 0x7F;

                self.pc += 1;
            }

            // SETC
            0x17 => {
                self.c_flag = true;

                self.pc += 1;
            }

            // CLRC
            0x18 => {
                self.c_flag = false;

                self.pc += 1;
            }

            // TRFNC
            0x19 => {
                self.c_flag = self.n_flag;

                self.pc += 1;
            }

            // BZ0
            0x1A => {
                if self.z_flag {
                    self.pc += 1;
                } else {
                    self.pc = operand;
                }
            }

            // BZ1
            0x1B => {
                if self.z_flag {
                    self.pc = operand;
                } else {
                    self.pc += 1;
                }
            }

            // BC0
            0x1C => {
                if self.c_flag {
                    self.pc += 1;
                } else {
                    self.pc = operand;
                }
            }

            // BC1
            0x1D => {
                if self.c_flag {
                    self.pc = operand;
                } else {
                    self.pc += 1;
                }
            }

            // BV0
            0x1E => {
                if self.v_flag {
                    self.pc += 1;
                } else {
                    self.pc = operand;
                }
            }

            // BV1
            0x1F => {
                if self.v_flag {
                    self.pc = operand;
                } else {
                    self.pc += 1;
                }
            }

            // BN0
            0x20 => {
                if self.n_flag {
                    self.pc += 1;
                } else {
                    self.pc = operand;
                }
            }

            // BN1
            0x21 => {
                if self.n_flag {
                    self.pc = operand;
                } else {
                    self.pc += 1;
                }
            }

            // BRA
            0x22 => {
                self.pc = operand;
            }

            // NOP
            0x3F => {
                self.pc += 1;
            }

            _ => {
                panic!("Unknown opcode {opcode}");
            }
        }
    }

    pub fn print_status(&self) {
        for y in 0..8 {
            for x in 0..8 {
                print!("{:3x}", self.ram[y * 8 + x]);
            }
            println!();
        }

        println!();
        println!("Program Counter : {:x}", self.pc);
        println!("Opcode : {:x}", self.rom[self.pc as usize] >> 8);
        println!("Operand : {:x}", self.rom[self.pc as usize] & 0xFF);
    }
}
