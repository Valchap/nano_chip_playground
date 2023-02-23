use crate::instruction_generator::{generate_instruction, Instruction, Parameter};

pub fn parse(text: &str) -> Result<Vec<u16>, String> {
    let mut binary = Vec::<u16>::new();

    let mut line_n = 1;

    for line in text.lines() {
        match parse_line(line) {
            Ok(line_binary) => {
                if let Some(bin) = line_binary {
                    binary.push(bin);
                }
            }
            Err(line_error) => return Err(format!("Error line {line_n} : {line_error}")),
        }

        line_n += 1;
    }

    Ok(binary)
}

fn parse_line(line: &str) -> Result<Option<u16>, String> {
    let mut words = line.split(';').next().unwrap().split_whitespace();

    let mut parameters = Vec::<Parameter>::new();

    if let Some(instruction_str) = words.next() {
        match parse_instruction(instruction_str) {
            Ok(instruction) => {
                for parameter_str in words {
                    match parse_parameter(parameter_str) {
                        Ok(parameter) => {
                            parameters.push(parameter);
                        }

                        Err(error) => return Err(error),
                    }
                }

                match generate_instruction(&instruction, &parameters) {
                    Ok(line_binary) => Ok(Some(line_binary)),

                    Err(line_error) => Err(line_error),
                }
            }

            Err(error) => Err(error),
        }
    } else {
        Ok(None)
    }
}

fn parse_instruction(word: &str) -> Result<Instruction, String> {
    match word {
        "ST" => Ok(Instruction::St),
        "LD" => Ok(Instruction::Ld),
        "AND" => Ok(Instruction::And),
        "OR" => Ok(Instruction::Or),
        "XOR" => Ok(Instruction::Xor),
        "ROL" => Ok(Instruction::Rol),
        "ROR" => Ok(Instruction::Ror),
        "ADD" => Ok(Instruction::Add),
        "ADC" => Ok(Instruction::Adc),
        "NEG" => Ok(Instruction::Neg),
        "INC" => Ok(Instruction::Inc),
        "DEC" => Ok(Instruction::Dec),
        "SETC" => Ok(Instruction::Setc),
        "CLRC" => Ok(Instruction::Clrc),
        "TRFNC" => Ok(Instruction::Trfnc),
        "BZ0" => Ok(Instruction::Bz0),
        "BZ1" => Ok(Instruction::Bz1),
        "BC0" => Ok(Instruction::Bc0),
        "BC1" => Ok(Instruction::Bc1),
        "BV0" => Ok(Instruction::Bv0),
        "BV1" => Ok(Instruction::Bv1),
        "BN0" => Ok(Instruction::Bn0),
        "BN1" => Ok(Instruction::Bn1),
        "BRA" => Ok(Instruction::Bra),
        "NOP" => Ok(Instruction::Nop),
        _ => Err("Unknown instruction name".to_owned()),
    }
}

fn parse_parameter(word: &str) -> Result<Parameter, String> {
    if word == "ACC" {
        Ok(Parameter::Acc)
    } else if let Some(n_str) = word.strip_suffix('A') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 256 {
                Ok(Parameter::Addr(n as u8))
            } else {
                Err("Address can only take values up to 255".to_owned())
            }
        } else {
            Err("Can't parse Address".to_owned())
        }
    } else if let Some(n_str) = word.strip_suffix('C') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 256 {
                Ok(Parameter::Const(n as u8))
            } else {
                Err("Constant can only take values up to 255".to_owned())
            }
        } else {
            Err("Can't parse constant".to_owned())
        }
    } else {
        Err("Unknown parameter type".to_owned())
    }
}
