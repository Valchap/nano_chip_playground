use crate::instruction_generator::generate_instruction;
use crate::syntax_tree::Instruction;
use crate::syntax_tree::Opcode;
use crate::syntax_tree::Parameter;
use crate::syntax_tree::SyntaxTree;
use crate::syntax_tree::Value;
use crate::syntax_tree::ValueType;

pub fn parse(text: &str) -> Result<Vec<u16>, String> {
    let mut binary = Vec::<u16>::new();

    let mut line_n = 1;

    let mut syntax_tree = SyntaxTree::new();

    for line in text.lines() {
        if let Err(line_error) = parse_line(line, &mut syntax_tree) {
            return Err(format!("Error line {line_n} : {line_error}"));
        }

        line_n += 1;
    }

    match syntax_tree.generate_instructions() {
        Ok(instructions) => {
            for instruction in &instructions {
                match generate_instruction(instruction) {
                    Ok(bin) => {
                        binary.push(bin);
                    }
                    Err(errmsg) => {
                        return Err(errmsg);
                    }
                }
            }
        }
        Err(err_msg) => {
            return Err(err_msg);
        }
    }

    Ok(binary)
}

fn parse_line(line: &str, syntax_tree: &mut SyntaxTree) -> Result<(), String> {
    let mut words = line.split(';').next().unwrap().split_whitespace();

    if let Some(instruction_str) = words.next() {
        if let Some(const_name) = instruction_str.strip_prefix('$') {
            if let Some(value_str) = words.next() {
                if let Ok(value) = value_str.parse::<u64>() {
                    if value < 256 {
                        syntax_tree.add_const(const_name, value as u8)?;
                    } else {
                        return Err("constant values can't be greater than 255".to_owned());
                    }
                } else {
                    return Err("Can't parse constant value".to_owned());
                }
            } else {
                return Err("You must specify a value after declaring a constant".to_owned());
            }
        } else if let Some(label_name) = instruction_str.strip_prefix(':') {
            syntax_tree.add_label(label_name)?;
        } else {
            match parse_instruction(instruction_str) {
                Ok(instruction) => {
                    let mut parameters = Vec::<Parameter>::new();
                    for parameter_str in words {
                        match parse_parameter(parameter_str) {
                            Ok(parameter) => {
                                parameters.push(parameter);
                            }

                            Err(error) => return Err(error),
                        }
                    }

                    syntax_tree.add_instruction(Instruction::new(instruction, parameters))?;
                }

                Err(error) => return Err(error),
            }
        }
    }

    Ok(())
}

fn parse_instruction(word: &str) -> Result<Opcode, String> {
    match word {
        "ST" => Ok(Opcode::St),
        "LD" => Ok(Opcode::Ld),
        "AND" => Ok(Opcode::And),
        "OR" => Ok(Opcode::Or),
        "XOR" => Ok(Opcode::Xor),
        "ROL" => Ok(Opcode::Rol),
        "ROR" => Ok(Opcode::Ror),
        "ADD" => Ok(Opcode::Add),
        "ADC" => Ok(Opcode::Adc),
        "NEG" => Ok(Opcode::Neg),
        "INC" => Ok(Opcode::Inc),
        "DEC" => Ok(Opcode::Dec),
        "SETC" => Ok(Opcode::Setc),
        "CLRC" => Ok(Opcode::Clrc),
        "TRFNC" => Ok(Opcode::Trfnc),
        "BZ0" => Ok(Opcode::Bz0),
        "BZ1" => Ok(Opcode::Bz1),
        "BC0" => Ok(Opcode::Bc0),
        "BC1" => Ok(Opcode::Bc1),
        "BV0" => Ok(Opcode::Bv0),
        "BV1" => Ok(Opcode::Bv1),
        "BN0" => Ok(Opcode::Bn0),
        "BN1" => Ok(Opcode::Bn1),
        "BRA" => Ok(Opcode::Bra),
        "NOP" => Ok(Opcode::Nop),
        _ => Err("Unknown instruction name".to_owned()),
    }
}

fn parse_parameter(word: &str) -> Result<Parameter, String> {
    if word == "ACC" {
        Ok(Parameter::Acc)
    } else {
        let mut parameter_str = word.to_string();
        let mut direct = true;

        if let Some(first_strip) = word.strip_prefix('[') {
            if let Some(second_strip) = first_strip.strip_suffix(']') {
                parameter_str = second_strip.to_owned();
                direct = false;
            } else {
                return Err("Can't parse parameter, did you forget a closing ] ?".to_owned());
            }
        }

        if let Ok(n) = parameter_str.parse::<u64>() {
            if n < 256 {
                Ok(Parameter::Value(Value::new(
                    direct,
                    ValueType::Raw(n as u8),
                )))
            } else {
                Err("Values can't be over 255".to_owned())
            }
        } else if let Some(const_name) = parameter_str.strip_prefix('$') {
            Ok(Parameter::Value(Value::new(
                direct,
                ValueType::Const(const_name.to_owned()),
            )))
        } else if let Some(label_name) = parameter_str.strip_prefix(':') {
            Ok(Parameter::Value(Value::new(
                direct,
                ValueType::Label(label_name.to_owned()),
            )))
        } else {
            Err("Can't parse parameter".to_owned())
        }
    }
}
