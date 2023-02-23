#![cfg(test)]

use crate::parser::parse;

#[test]
fn test_st() {
    assert_eq!(parse("ST 42A"), Ok(vec![0x012A]));
}

#[test]
fn test_ld() {
    assert_eq!(parse("LD 42C"), Ok(vec![0x022A]));
    assert_eq!(parse("LD 42A"), Ok(vec![0x032A]));
}

#[test]
fn test_and() {
    assert_eq!(parse("AND 42C"), Ok(vec![0x042A]));
    assert_eq!(parse("AND 42A"), Ok(vec![0x052A]));
}

#[test]
fn test_or() {
    assert_eq!(parse("OR 42C"), Ok(vec![0x062A]));
    assert_eq!(parse("OR 42A"), Ok(vec![0x072A]));
}

#[test]
fn test_xor() {
    assert_eq!(parse("XOR 42C"), Ok(vec![0x082A]));
    assert_eq!(parse("XOR 42A"), Ok(vec![0x092A]));
}

#[test]
fn test_shl() {
    assert_eq!(parse("ROL ACC"), Ok(vec![0x0A00]));
}

#[test]
fn test_shr() {
    assert_eq!(parse("ROR ACC"), Ok(vec![0x0B00]));
}

#[test]
fn test_add() {
    assert_eq!(parse("ADD 42C"), Ok(vec![0x0C2A]));
    assert_eq!(parse("ADD 42A"), Ok(vec![0x0D2A]));
}

#[test]
fn test_adc() {
    assert_eq!(parse("ADC 42C"), Ok(vec![0x0E2A]));
    assert_eq!(parse("ADC 42A"), Ok(vec![0x0F2A]));
}

#[test]
fn test_neg() {
    assert_eq!(parse("NEG ACC"), Ok(vec![0x1000]));
    assert_eq!(parse("NEG 42C"), Ok(vec![0x112A]));
    assert_eq!(parse("NEG 42A"), Ok(vec![0x122A]));
}

#[test]
fn test_inc() {
    assert_eq!(parse("INC ACC"), Ok(vec![0x1300]));
    assert_eq!(parse("INC 42A"), Ok(vec![0x142A]));
}

#[test]
fn test_dec() {
    assert_eq!(parse("DEC ACC"), Ok(vec![0x1500]));
    assert_eq!(parse("DEC 42A"), Ok(vec![0x162A]));
}

#[test]
fn test_setc() {
    assert_eq!(parse("SETC"), Ok(vec![0x1700]));
}

#[test]
fn test_clrc() {
    assert_eq!(parse("CLRC"), Ok(vec![0x1800]));
}

#[test]
fn test_trfnc() {
    assert_eq!(parse("TRFNC"), Ok(vec!(0x1900)));
}

#[test]
fn test_bz0() {
    assert_eq!(parse("BZ0 42A"), Ok(vec![0x1A2A]));
}

#[test]
fn test_bz1() {
    assert_eq!(parse("BZ1 42A"), Ok(vec![0x1B2A]));
}

#[test]
fn test_bc0() {
    assert_eq!(parse("BC0 42A"), Ok(vec![0x1C2A]));
}

#[test]
fn test_bc1() {
    assert_eq!(parse("BC1 42A"), Ok(vec![0x1D2A]));
}

#[test]
fn test_bv0() {
    assert_eq!(parse("BV0 42A"), Ok(vec![0x1E2A]));
}

#[test]
fn test_bv1() {
    assert_eq!(parse("BV1 42A"), Ok(vec![0x1F2A]));
}

#[test]
fn test_bn0() {
    assert_eq!(parse("BN0 42A"), Ok(vec![0x202A]));
}

#[test]
fn test_bn1() {
    assert_eq!(parse("BN1 42A"), Ok(vec![0x212A]));
}

#[test]
fn test_bra() {
    assert_eq!(parse("BRA 42A"), Ok(vec![0x222A]));
}

#[test]
fn test_nop() {
    assert_eq!(parse("NOP"), Ok(vec![0x3F00]));
}

#[test]
fn test_comment() {
    assert_eq!(
        parse(
            "
            ;First comment
            NOP; second comment
            ; Last comment
        "
        ),
        Ok(vec![0x3F00])
    );
}
