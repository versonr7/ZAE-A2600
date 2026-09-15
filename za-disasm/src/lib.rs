//! za-disasm — Zero-allocation disassembler for MOS 6502
//!
//! Covering the full official 6502 instruction set (151 opcodes).
//! `no_std` compatible.

#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Imp,  // Implied: 1 byte
    Acc,  // Accumulator: 1 byte
    Imm,  // Immediate: 2 bytes  #$XX
    Zp,   // Zero Page: 2 bytes  $XX
    ZpX,  // Zero Page,X: 2 bytes  $XX,X
    ZpY,  // Zero Page,Y: 2 bytes  $XX,Y
    Abs,  // Absolute: 3 bytes  $XXXX
    AbsX, // Absolute,X: 3 bytes  $XXXX,X
    AbsY, // Absolute,Y: 3 bytes  $XXXX,Y
    Ind,  // Indirect (JMP only): 3 bytes  ($XXXX)
    IndX, // (Indirect,X): 2 bytes  ($XX,X)
    IndY, // (Indirect),Y: 2 bytes  ($XX),Y
    Rel,  // Relative (branches): 2 bytes
}

pub struct OpEntry {
    pub mnemonic: &'static str,
    pub mode: AddressingMode,
}

pub const fn op(m: &'static str, mode: AddressingMode) -> OpEntry {
    OpEntry { mnemonic: m, mode }
}

pub const IMP: AddressingMode = AddressingMode::Imp;
pub const ACC: AddressingMode = AddressingMode::Acc;
pub const IMM: AddressingMode = AddressingMode::Imm;
pub const ZP: AddressingMode = AddressingMode::Zp;
pub const ZPX: AddressingMode = AddressingMode::ZpX;
pub const ZPY: AddressingMode = AddressingMode::ZpY;
pub const ABS: AddressingMode = AddressingMode::Abs;
pub const ABX: AddressingMode = AddressingMode::AbsX;
pub const ABY: AddressingMode = AddressingMode::AbsY;
pub const IND: AddressingMode = AddressingMode::Ind;
pub const IZX: AddressingMode = AddressingMode::IndX;
pub const IZY: AddressingMode = AddressingMode::IndY;
pub const REL: AddressingMode = AddressingMode::Rel;

// Complete official 6502 opcode table (256 entries, illegal opcodes marked "???")
pub static OPCODES: [OpEntry; 256] = [
    // 0x00-0x0F
    op("BRK", IMP),
    op("ORA", IZX),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("ORA", ZP),
    op("ASL", ZP),
    op("???", IMP),
    op("PHP", IMP),
    op("ORA", IMM),
    op("ASL", ACC),
    op("???", IMP),
    op("???", IMP),
    op("ORA", ABS),
    op("ASL", ABS),
    op("???", IMP),
    // 0x10-0x1F
    op("BPL", REL),
    op("ORA", IZY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("ORA", ZPX),
    op("ASL", ZPX),
    op("???", IMP),
    op("CLC", IMP),
    op("ORA", ABY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("ORA", ABX),
    op("ASL", ABX),
    op("???", IMP),
    // 0x20-0x2F
    op("JSR", ABS),
    op("AND", IZX),
    op("???", IMP),
    op("???", IMP),
    op("BIT", ZP),
    op("AND", ZP),
    op("ROL", ZP),
    op("???", IMP),
    op("PLP", IMP),
    op("AND", IMM),
    op("ROL", ACC),
    op("???", IMP),
    op("BIT", ABS),
    op("AND", ABS),
    op("ROL", ABS),
    op("???", IMP),
    // 0x30-0x3F
    op("BMI", REL),
    op("AND", IZY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("AND", ZPX),
    op("ROL", ZPX),
    op("???", IMP),
    op("SEC", IMP),
    op("AND", ABY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("AND", ABX),
    op("ROL", ABX),
    op("???", IMP),
    // 0x40-0x4F
    op("RTI", IMP),
    op("EOR", IZX),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("EOR", ZP),
    op("LSR", ZP),
    op("???", IMP),
    op("PHA", IMP),
    op("EOR", IMM),
    op("LSR", ACC),
    op("???", IMP),
    op("JMP", ABS),
    op("EOR", ABS),
    op("LSR", ABS),
    op("???", IMP),
    // 0x50-0x5F
    op("BVC", REL),
    op("EOR", IZY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("EOR", ZPX),
    op("LSR", ZPX),
    op("???", IMP),
    op("CLI", IMP),
    op("EOR", ABY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("EOR", ABX),
    op("LSR", ABX),
    op("???", IMP),
    // 0x60-0x6F
    op("RTS", IMP),
    op("ADC", IZX),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("ADC", ZP),
    op("ROR", ZP),
    op("???", IMP),
    op("PLA", IMP),
    op("ADC", IMM),
    op("ROR", ACC),
    op("???", IMP),
    op("JMP", IND),
    op("ADC", ABS),
    op("ROR", ABS),
    op("???", IMP),
    // 0x70-0x7F
    op("BVS", REL),
    op("ADC", IZY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("ADC", ZPX),
    op("ROR", ZPX),
    op("???", IMP),
    op("SEI", IMP),
    op("ADC", ABY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("ADC", ABX),
    op("ROR", ABX),
    op("???", IMP),
    // 0x80-0x8F
    op("???", IMP),
    op("STA", IZX),
    op("???", IMP),
    op("???", IMP),
    op("STY", ZP),
    op("STA", ZP),
    op("STX", ZP),
    op("???", IMP),
    op("DEY", IMP),
    op("???", IMM),
    op("TXA", IMP),
    op("???", IMP),
    op("STY", ABS),
    op("STA", ABS),
    op("STX", ABS),
    op("???", IMP),
    // 0x90-0x9F
    op("BCC", REL),
    op("STA", IZY),
    op("???", IMP),
    op("???", IMP),
    op("STY", ZPX),
    op("STA", ZPX),
    op("STX", ZPY),
    op("???", IMP),
    op("TYA", IMP),
    op("STA", ABY),
    op("TXS", IMP),
    op("???", IMP),
    op("???", IMP),
    op("STA", ABX),
    op("???", IMP),
    op("???", IMP),
    // 0xA0-0xAF
    op("LDY", IMM),
    op("LDA", IZX),
    op("LDX", IMM),
    op("???", IMP),
    op("LDY", ZP),
    op("LDA", ZP),
    op("LDX", ZP),
    op("???", IMP),
    op("TAY", IMP),
    op("LDA", IMM),
    op("TAX", IMP),
    op("???", IMP),
    op("LDY", ABS),
    op("LDA", ABS),
    op("LDX", ABS),
    op("???", IMP),
    // 0xB0-0xBF
    op("BCS", REL),
    op("LDA", IZY),
    op("???", IMP),
    op("???", IMP),
    op("LDY", ZPX),
    op("LDA", ZPX),
    op("LDX", ZPY),
    op("???", IMP),
    op("CLV", IMP),
    op("LDA", ABY),
    op("TSX", IMP),
    op("???", IMP),
    op("LDY", ABX),
    op("LDA", ABX),
    op("LDX", ABY),
    op("???", IMP),
    // 0xC0-0xCF
    op("CPY", IMM),
    op("CMP", IZX),
    op("???", IMP),
    op("???", IMP),
    op("CPY", ZP),
    op("CMP", ZP),
    op("DEC", ZP),
    op("???", IMP),
    op("INY", IMP),
    op("CMP", IMM),
    op("DEX", IMP),
    op("???", IMP),
    op("CPY", ABS),
    op("CMP", ABS),
    op("DEC", ABS),
    op("???", IMP),
    // 0xD0-0xDF
    op("BNE", REL),
    op("CMP", IZY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("CMP", ZPX),
    op("DEC", ZPX),
    op("???", IMP),
    op("CLD", IMP),
    op("CMP", ABY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("CMP", ABX),
    op("DEC", ABX),
    op("???", IMP),
    // 0xE0-0xEF
    op("CPX", IMM),
    op("SBC", IZX),
    op("???", IMP),
    op("???", IMP),
    op("CPX", ZP),
    op("SBC", ZP),
    op("INC", ZP),
    op("???", IMP),
    op("INX", IMP),
    op("SBC", IMM),
    op("NOP", IMP),
    op("???", IMP),
    op("CPX", ABS),
    op("SBC", ABS),
    op("INC", ABS),
    op("???", IMP),
    // 0xF0-0xFF
    op("BEQ", REL),
    op("SBC", IZY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("SBC", ZPX),
    op("INC", ZPX),
    op("???", IMP),
    op("SED", IMP),
    op("SBC", ABY),
    op("???", IMP),
    op("???", IMP),
    op("???", IMP),
    op("SBC", ABX),
    op("INC", ABX),
    op("???", IMP),
];

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub addr: u16,
    pub bytes: [u8; 3],
    pub size: u8,
    pub mnemonic: &'static str,
    pub mode: AddressingMode,
}

pub const fn mode_size(mode: AddressingMode) -> u8 {
    match mode {
        IMP | ACC => 1,
        IMM | ZP | ZPX | ZPY | IZX | IZY | REL => 2,
        ABS | ABX | ABY | IND => 3,
    }
}

pub fn decode(buf: &[u8], addr: u16) -> Instruction {
    let b0 = if buf.len() > 0 { buf[0] } else { 0 };
    let b1 = if buf.len() > 1 { buf[1] } else { 0 };
    let b2 = if buf.len() > 2 { buf[2] } else { 0 };
    let entry = &OPCODES[b0 as usize];
    Instruction {
        addr,
        bytes: [b0, b1, b2],
        size: mode_size(entry.mode),
        mnemonic: entry.mnemonic,
        mode: entry.mode,
    }
}

fn write_str(out: &mut [u8], p: &mut usize, s: &str) {
    for &c in s.as_bytes() {
        if *p < out.len() {
            out[*p] = c;
            *p += 1;
        }
    }
}

fn write_hex_u8(out: &mut [u8], p: &mut usize, v: u8) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    if *p < out.len() {
        out[*p] = HEX[(v >> 4) as usize];
        *p += 1;
    }
    if *p < out.len() {
        out[*p] = HEX[(v & 0x0F) as usize];
        *p += 1;
    }
}

fn write_hex_u16(out: &mut [u8], p: &mut usize, v: u16) {
    write_hex_u8(out, p, (v >> 8) as u8);
    write_hex_u8(out, p, (v & 0xFF) as u8);
}

/// Formats the instruction into `out` and returns bytes written.
pub fn format(out: &mut [u8], ins: &Instruction) -> usize {
    let mut p = 0;
    write_str(out, &mut p, ins.mnemonic);
    let b1 = ins.bytes[1];
    let b2 = ins.bytes[2];

    match ins.mode {
        IMP => {}
        ACC => write_str(out, &mut p, " A"),
        IMM => {
            write_str(out, &mut p, " #$");
            write_hex_u8(out, &mut p, b1);
        }
        ZP => {
            write_str(out, &mut p, " $");
            write_hex_u8(out, &mut p, b1);
        }
        ZPX => {
            write_str(out, &mut p, " $");
            write_hex_u8(out, &mut p, b1);
            write_str(out, &mut p, ",X");
        }
        ZPY => {
            write_str(out, &mut p, " $");
            write_hex_u8(out, &mut p, b1);
            write_str(out, &mut p, ",Y");
        }
        ABS => {
            let w = ((b2 as u16) << 8) | b1 as u16;
            write_str(out, &mut p, " $");
            write_hex_u16(out, &mut p, w);
        }
        ABX => {
            let w = ((b2 as u16) << 8) | b1 as u16;
            write_str(out, &mut p, " $");
            write_hex_u16(out, &mut p, w);
            write_str(out, &mut p, ",X");
        }
        ABY => {
            let w = ((b2 as u16) << 8) | b1 as u16;
            write_str(out, &mut p, " $");
            write_hex_u16(out, &mut p, w);
            write_str(out, &mut p, ",Y");
        }
        IND => {
            let w = ((b2 as u16) << 8) | b1 as u16;
            write_str(out, &mut p, " ($");
            write_hex_u16(out, &mut p, w);
            write_str(out, &mut p, ")");
        }
        IZX => {
            write_str(out, &mut p, " ($");
            write_hex_u8(out, &mut p, b1);
            write_str(out, &mut p, ",X)");
        }
        IZY => {
            write_str(out, &mut p, " ($");
            write_hex_u8(out, &mut p, b1);
            write_str(out, &mut p, "),Y");
        }
        REL => {
            let off = b1 as i8 as i32;
            let target = ((ins.addr as i32) + 2 + off) as u16;
            write_str(out, &mut p, " $");
            write_hex_u16(out, &mut p, target);
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(buf: &[u8]) -> &str {
        // alloc is available only for tests; unsafe but works
        let _ = buf;
        "TODO"
    }

    #[test]
    fn test_decode_lda_immediate() {
        let ins = decode(&[0xA9, 0x42], 0xF000);
        assert_eq!(ins.mnemonic, "LDA");
        assert_eq!(ins.mode, IMM);
        assert_eq!(ins.size, 2);
    }

    #[test]
    fn test_decode_jmp_absolute() {
        let ins = decode(&[0x4C, 0x00, 0xF0], 0xF000);
        assert_eq!(ins.mnemonic, "JMP");
        assert_eq!(ins.mode, ABS);
        assert_eq!(ins.size, 3);
    }

    #[test]
    fn test_decode_bne_relative() {
        let ins = decode(&[0xD0, 0xFB], 0xF02F);
        assert_eq!(ins.mnemonic, "BNE");
        assert_eq!(ins.mode, REL);
        let mut out = [0u8; 32];
        let n = format(&mut out, &ins);
        assert_eq!(&out[..n], b"BNE $F02C");
    }

    #[test]
    fn test_format_lda_imm() {
        let ins = decode(&[0xA9, 0x42], 0xF000);
        let mut out = [0u8; 32];
        let n = format(&mut out, &ins);
        assert_eq!(&out[..n], b"LDA #$42");
    }

    #[test]
    fn test_format_implied() {
        let ins = decode(&[0xEA], 0xF000);
        let mut out = [0u8; 32];
        let n = format(&mut out, &ins);
        assert_eq!(&out[..n], b"NOP");
    }
}
