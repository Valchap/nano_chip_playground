use crate::syntax_tree::{Instruction, Opcode, Parameter, ValueType};

fn generate_st(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ST instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Err("ST instruction only supports address value".to_owned())
                } else {
                    Ok(0x01 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("ST takes an address as parameter".to_owned()),
    }
}

fn generate_ld(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("LD instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x02 << 8 | operand as u16)
                } else {
                    Ok(0x03 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("LD takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_and(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("AND instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x04 << 8 | operand as u16)
                } else {
                    Ok(0x05 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("AND takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_or(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("OR instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x06 << 8 | operand as u16)
                } else {
                    Ok(0x07 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("OR takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_xor(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("XOR instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x08 << 8 | operand as u16)
                } else {
                    Ok(0x09 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("XOR takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_rol(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ROL instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Acc => Ok(0x0A << 8),
        _ => Err("ROL takes acc as parameter".to_owned()),
    }
}

fn generate_ror(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ROR instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Acc => Ok(0x0B << 8),
        _ => Err("ROR takes acc as parameter".to_owned()),
    }
}

fn generate_add(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ADD instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x0C << 8 | operand as u16)
                } else {
                    Ok(0x0D << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("ADD takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_adc(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ADC instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x0E << 8 | operand as u16)
                } else {
                    Ok(0x0F << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("ADC takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_neg(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("NEG instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Acc => Ok(0x10 << 8),

        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x11 << 8 | operand as u16)
                } else {
                    Ok(0x12 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
    }
}

fn generate_inc(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("INC instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Acc => Ok(0x13 << 8),

        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Err("INC only support address values".to_owned())
                } else {
                    Ok(0x14 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
    }
}

fn generate_dec(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("DEC instruction always takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Acc => Ok(0x15 << 8),

        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Err("DEC only support address values".to_owned())
                } else {
                    Ok(0x16 << 8 | operand as u16)
                }
            } else {
                unreachable!();
            }
        }
    }
}

fn generate_setc(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("SETC instruction takes no parameter".to_owned());
    }

    Ok(0x17 << 8)
}

fn generate_clrc(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("CLRC instruction takes no parameter".to_owned());
    }

    Ok(0x18 << 8)
}

fn generate_trfnc(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("TRFNC instruction takes no prameter".to_owned());
    }

    Ok(0x19 << 8)
}

fn generate_bz0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BZ0 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x1A << 8 | operand as u16)
                } else {
                    Err("BZ0 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BZ0 takes an address as parameter".to_owned()),
    }
}

fn generate_bz1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BZ1 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x1B << 8 | operand as u16)
                } else {
                    Err("BZ1 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BZ1 takes an address as parameter".to_owned()),
    }
}

fn generate_bc0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BC0 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x1C << 8 | operand as u16)
                } else {
                    Err("BC0 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BC0 takes an address as parameter".to_owned()),
    }
}

fn generate_bc1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BC1 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x1D << 8 | operand as u16)
                } else {
                    Err("BC1 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BC1 takes an address as parameter".to_owned()),
    }
}

fn generate_bv0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BV0 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x1E << 8 | operand as u16)
                } else {
                    Err("BV0 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BV0 takes an address as parameter".to_owned()),
    }
}

fn generate_bv1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BV1 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x1F << 8 | operand as u16)
                } else {
                    Err("BV1 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BV1 takes an address as parameter".to_owned()),
    }
}

fn generate_bn0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BN0 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x20 << 8 | operand as u16)
                } else {
                    Err("BN0 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BN0 takes an address as parameter".to_owned()),
    }
}

fn generate_bn1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BN1 instruction takes a single parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x21 << 8 | operand as u16)
                } else {
                    Err("BN1 only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BN1 takes an address as parameter".to_owned()),
    }
}

fn generate_bra(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BRA instruction takes no parameter".to_owned());
    }

    match &parameters[0] {
        Parameter::Value(value) => {
            if let ValueType::Raw(operand) = value.value_type {
                if value.direct {
                    Ok(0x22 << 8 | operand as u16)
                } else {
                    Err("BRA only support direct values".to_owned())
                }
            } else {
                unreachable!();
            }
        }
        _ => Err("BRA takes an address as parameter".to_owned()),
    }
}

fn generate_nop(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("NOP instruction takes no parameter".to_owned());
    }

    Ok(0x3F << 8)
}

pub fn generate_instruction(instruction: &Instruction) -> Result<u16, String> {
    (match instruction.opcode {
        Opcode::St => generate_st,
        Opcode::Ld => generate_ld,
        Opcode::And => generate_and,
        Opcode::Or => generate_or,
        Opcode::Xor => generate_xor,
        Opcode::Rol => generate_rol,
        Opcode::Ror => generate_ror,
        Opcode::Add => generate_add,
        Opcode::Adc => generate_adc,
        Opcode::Neg => generate_neg,
        Opcode::Inc => generate_inc,
        Opcode::Dec => generate_dec,
        Opcode::Setc => generate_setc,
        Opcode::Clrc => generate_clrc,
        Opcode::Trfnc => generate_trfnc,
        Opcode::Bz0 => generate_bz0,
        Opcode::Bz1 => generate_bz1,
        Opcode::Bc0 => generate_bc0,
        Opcode::Bc1 => generate_bc1,
        Opcode::Bv0 => generate_bv0,
        Opcode::Bv1 => generate_bv1,
        Opcode::Bn0 => generate_bn0,
        Opcode::Bn1 => generate_bn1,
        Opcode::Bra => generate_bra,
        Opcode::Nop => generate_nop,
    })(&instruction.param)
}
