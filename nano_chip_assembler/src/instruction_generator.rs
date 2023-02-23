pub enum Instruction {
    St,
    Ld,
    And,
    Or,
    Xor,
    Rol,
    Ror,
    Add,
    Adc,
    Neg,
    Inc,
    Dec,
    Setc,
    Clrc,
    Trfnc,
    Bz0,
    Bz1,
    Bc0,
    Bc1,
    Bv0,
    Bv1,
    Bn0,
    Bn1,
    Bra,
    Nop,
}

pub enum Parameter {
    Acc,
    Addr(u8),
    Const(u8),
}

fn generate_st(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ST instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x01 << 8 | address as u16),
        _ => Err("ST takes an address as parameter".to_owned()),
    }
}

fn generate_ld(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("LD instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Const(constant) => Ok(0x02 << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x03 << 8 | address as u16),
        _ => Err("LD takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_and(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("AND instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Const(constant) => Ok(0x04 << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x05 << 8 | address as u16),
        _ => Err("AND takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_or(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("OR instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Const(constant) => Ok(0x06 << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x07 << 8 | address as u16),
        _ => Err("OR takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_xor(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("XOR instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Const(constant) => Ok(0x08 << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x09 << 8 | address as u16),
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

    match parameters[0] {
        Parameter::Const(constant) => Ok(0x0C << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x0D << 8 | address as u16),
        _ => Err("ADD takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_adc(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("ADC instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Const(constant) => Ok(0x0E << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x0F << 8 | address as u16),
        _ => Err("ADC takes a constant or an address as parameter".to_owned()),
    }
}

fn generate_neg(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("NEG instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Acc => Ok(0x10 << 8),
        Parameter::Const(constant) => Ok(0x11 << 8 | constant as u16),
        Parameter::Addr(address) => Ok(0x12 << 8 | address as u16),
    }
}

fn generate_inc(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("INC instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Acc => Ok(0x13 << 8),
        Parameter::Addr(address) => Ok(0x14 << 8 | address as u16),
        _ => Err("INC takes acc or an address as parameter".to_owned()),
    }
}

fn generate_dec(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("DEC instruction always takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Acc => Ok(0x15 << 8),
        Parameter::Addr(address) => Ok(0x16 << 8 | address as u16),
        _ => Err("DEC takes acc or an address as parameter".to_owned()),
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

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x1A << 8 | address as u16),
        _ => Err("BZ0 takes an address as parameter".to_owned()),
    }
}

fn generate_bz1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BZ1 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x1B << 8 | address as u16),
        _ => Err("BZ1 takes an address as parameter".to_owned()),
    }
}

fn generate_bc0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BC0 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x1C << 8 | address as u16),
        _ => Err("BC0 takes an address as parameter".to_owned()),
    }
}

fn generate_bc1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BC1 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x1D << 8 | address as u16),
        _ => Err("BC1 takes an address as parameter".to_owned()),
    }
}

fn generate_bv0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BV0 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x1E << 8 | address as u16),
        _ => Err("BV0 takes an address as parameter".to_owned()),
    }
}

fn generate_bv1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BV1 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x1F << 8 | address as u16),
        _ => Err("BV1 takes an address as parameter".to_owned()),
    }
}

fn generate_bn0(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BN0 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x20 << 8 | address as u16),
        _ => Err("BN0 takes an address as parameter".to_owned()),
    }
}

fn generate_bn1(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BN1 instruction takes a single parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x21 << 8 | address as u16),
        _ => Err("BN1 takes an address as parameter".to_owned()),
    }
}

fn generate_bra(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BRA instruction takes no parameter".to_owned());
    }

    match parameters[0] {
        Parameter::Addr(address) => Ok(0x22 << 8 | address as u16),
        _ => Err("BRA takes an address as parameter".to_owned()),
    }
}

fn generate_nop(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("NOP instruction takes no parameter".to_owned());
    }

    Ok(0x3F << 8)
}

pub fn generate_instruction(
    instruction: &Instruction,
    parameters: &[Parameter],
) -> Result<u16, String> {
    match instruction {
        Instruction::St => generate_st(parameters),
        Instruction::Ld => generate_ld(parameters),
        Instruction::And => generate_and(parameters),
        Instruction::Or => generate_or(parameters),
        Instruction::Xor => generate_xor(parameters),
        Instruction::Rol => generate_rol(parameters),
        Instruction::Ror => generate_ror(parameters),
        Instruction::Add => generate_add(parameters),
        Instruction::Adc => generate_adc(parameters),
        Instruction::Neg => generate_neg(parameters),
        Instruction::Inc => generate_inc(parameters),
        Instruction::Dec => generate_dec(parameters),
        Instruction::Setc => generate_setc(parameters),
        Instruction::Clrc => generate_clrc(parameters),
        Instruction::Trfnc => generate_trfnc(parameters),
        Instruction::Bz0 => generate_bz0(parameters),
        Instruction::Bz1 => generate_bz1(parameters),
        Instruction::Bc0 => generate_bc0(parameters),
        Instruction::Bc1 => generate_bc1(parameters),
        Instruction::Bv0 => generate_bv0(parameters),
        Instruction::Bv1 => generate_bv1(parameters),
        Instruction::Bn0 => generate_bn0(parameters),
        Instruction::Bn1 => generate_bn1(parameters),
        Instruction::Bra => generate_bra(parameters),
        Instruction::Nop => generate_nop(parameters),
    }
}
