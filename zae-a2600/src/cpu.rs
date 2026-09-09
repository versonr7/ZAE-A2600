use crate::memory::Memory;

#[derive(Default, Clone, Copy)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8, // NV-BDIZC
    pub cycles: u32,
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0,
            status: 0x24, // I flag set
            cycles: 0,
        }
    }

    // أعلام الحالة
    const C: u8 = 0x01;
    const Z: u8 = 0x02;
    const I: u8 = 0x04; // إن احتجت
    const D: u8 = 0x08;
    const N: u8 = 0x80;
    const V: u8 = 0x40;

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }

    fn get_flag(&self, flag: u8) -> bool {
        self.status & flag != 0
    }

    fn set_z_n(&mut self, value: u8) {
        self.set_flag(Self::Z, value == 0);
        self.set_flag(Self::N, value & 0x80 != 0);
    }

    fn compare(&mut self, reg: u8, value: u8) {
        let result = reg.wrapping_sub(value);
        self.set_flag(Self::C, reg >= value);
        self.set_z_n(result);
    }

    fn adc(&mut self, value: u8) -> u32 {
        let carry = self.get_flag(Self::C) as u8;
        let result = self.a as u16 + value as u16 + carry as u16;
        self.set_flag(Self::C, result > 0xFF);
        self.set_flag(
            Self::V,
            (!(self.a ^ value) & (self.a ^ (result as u8)) & 0x80) != 0,
        );
        self.a = (result & 0xFF) as u8;
        self.set_z_n(self.a);
        0 // القيمة الحقيقية للدورات تحدد في الذراع
    }

    fn sbc(&mut self, value: u8) {
        let carry = self.get_flag(Self::C) as u8;
        let result = self.a as i16 - value as i16 - (1 - carry as i16);
        self.set_flag(Self::C, result >= 0);
        self.set_flag(
            Self::V,
            ((self.a ^ value) & (self.a ^ (result as u8)) & 0x80) != 0,
        );
        self.a = (result as u8) & 0xFF;
        self.set_z_n(self.a);
    }

    fn bit(&mut self, value: u8) {
        self.set_flag(Self::Z, (self.a & value) == 0);
        self.set_flag(Self::N, value & 0x80 != 0);
        self.set_flag(Self::V, value & 0x40 != 0);
    }

    fn read_byte(&mut self, mem: &mut Memory, addr: u16) -> u8 {
        mem.read(addr)
    }

    fn write_byte(&mut self, mem: &mut Memory, addr: u16, value: u8) {
        mem.write(addr, value);
    }

    fn read_word(&mut self, mem: &mut Memory, addr: u16) -> u16 {
        let lo = self.read_byte(mem, addr) as u16;
        let hi = self.read_byte(mem, addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    // أنماط عنونة
    fn immediate(&mut self, mem: &mut Memory) -> u8 {
        let val = self.read_byte(mem, self.pc);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    fn zero_page(&mut self, mem: &mut Memory) -> u16 {
        let addr = self.read_byte(mem, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        addr
    }

    fn absolute(&mut self, mem: &mut Memory) -> u16 {
        let addr = self.read_word(mem, self.pc);
        self.pc = self.pc.wrapping_add(2);
        addr
    }

    fn zero_page_x(&mut self, mem: &mut Memory) -> u16 {
        let base = self.read_byte(mem, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        base.wrapping_add(self.x as u16) & 0xFF
    }

    fn absolute_x(&mut self, mem: &mut Memory) -> u16 {
        let base = self.read_word(mem, self.pc);
        self.pc = self.pc.wrapping_add(2);
        base.wrapping_add(self.x as u16)
    }

    fn indirect_x(&mut self, mem: &mut Memory) -> u16 {
        let zp = self.read_byte(mem, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let addr = (zp.wrapping_add(self.x as u16) & 0xFF) as u16;
        self.read_word(mem, addr)
    }
    fn zero_page_y(&mut self, mem: &mut Memory) -> u16 {
        let base = self.read_byte(mem, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        base.wrapping_add(self.y as u16) & 0xFF
    }

    fn absolute_y(&mut self, mem: &mut Memory) -> u16 {
        let base = self.read_word(mem, self.pc);
        self.pc = self.pc.wrapping_add(2);
        base.wrapping_add(self.y as u16)
    }

    fn fetch_relative(&mut self, mem: &mut Memory) -> i8 {
        let offset = self.read_byte(mem, self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        offset
    }

    fn indirect_y(&mut self, mem: &mut Memory) -> u16 {
        let zp = self.read_byte(mem, self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let base = self.read_word(mem, zp);
        base.wrapping_add(self.y as u16)
    }

    pub fn reset(&mut self, mem: &mut Memory) {
    let lo = mem.read(0xFFFC);
    let hi = mem.read(0xFFFD);
    self.pc = ((hi as u16) << 8) | lo as u16;
    self.sp = 0xFD;
    self.status = 0x24;
    }

    pub fn step(&mut self, mem: &mut Memory) -> u32 {
        let opcode = self.read_byte(mem, self.pc);
        self.pc = self.pc.wrapping_add(1);
        let cycles = self.execute(opcode, mem);
        mem.tia.tick(cycles); // ← أضف
        self.cycles += cycles;
        cycles
    }

    pub fn run_for_cycles(&mut self, mem: &mut Memory, target_cycles: u32) -> u32 {
        let mut total = 0;
        while total < target_cycles {
            let cycles = self.step(mem);
            total += cycles;

            // إذا كان WSYNC مفعّلًا، أضف دورات حتى نهاية خط المسح
            if mem.tia.wsync {
                let remaining = 76 - (mem.tia.cycle_in_scanline % 76);
                mem.tia.tick(remaining);
                total += remaining;
                mem.tia.wsync = false;
            }
        }
        total
    }

    pub fn run_frame(&mut self, mem: &mut Memory) -> u32 {
        // إطار كامل: 262 خط مسح × 76 دورة
        self.run_for_cycles(mem, 19912)
    }

    pub fn execute(&mut self, opcode: u8, mem: &mut Memory) -> u32 {
        match opcode {
            0xA9 => {
                let v = self.immediate(mem);
                self.a = v;
                self.set_z_n(v);
                2
            }
            0xA5 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                3
            }
            0xB5 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                4
            }
            0xAD => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                4
            }
            0xBD => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                4
            }
            0xA1 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                6
            }
            0xB1 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                5
            }

            0x85 => {
                let a = self.zero_page(mem);
                self.write_byte(mem, a, self.a);
                3
            }
            0x95 => {
                let a = self.zero_page_x(mem);
                self.write_byte(mem, a, self.a);
                4
            }
            0x8D => {
                let a = self.absolute(mem);
                self.write_byte(mem, a, self.a);
                4
            }
            0x9D => {
                let a = self.absolute_x(mem);
                self.write_byte(mem, a, self.a);
                5
            }
            0x81 => {
                let a = self.indirect_x(mem);
                self.write_byte(mem, a, self.a);
                6
            }
            0x91 => {
                let a = self.indirect_y(mem);
                self.write_byte(mem, a, self.a);
                6
            }

            0x4C => {
                let a = self.absolute(mem);
                self.pc = a;
                3
            }
            0x20 => {
                let target = self.absolute(mem);
                let ret = self.pc;
                self.push_word(mem, ret);
                self.pc = target;
                6
            }
            0x60 => {
                self.pc = self.pull_word(mem);
                6
            }
            0xF0 => {
                let addr = self.absolute(mem);
                if self.get_flag(Self::Z) {
                    self.pc = addr;
                    4
                } else {
                    2
                }
            }
            0xD0 => {
                let addr = self.absolute(mem);
                if !self.get_flag(Self::Z) {
                    self.pc = addr;
                    4
                } else {
                    2
                }
            }
            0x18 => {
                self.set_flag(Self::C, false);
                2
            }
            0x38 => {
                self.set_flag(Self::C, true);
                2
            }
            // INX / INY / DEX / DEY
            0xE8 => {
                self.x = self.x.wrapping_add(1);
                self.set_z_n(self.x);
                2
            }
            0xC8 => {
                self.y = self.y.wrapping_add(1);
                self.set_z_n(self.y);
                2
            }
            0xCA => {
                self.x = self.x.wrapping_sub(1);
                self.set_z_n(self.x);
                2
            }
            0x88 => {
                self.y = self.y.wrapping_sub(1);
                self.set_z_n(self.y);
                2
            }

            // CMP (Immediate / ZeroPage / Absolute)
            0xC9 => {
                let v = self.immediate(mem);
                self.compare(self.a, v);
                2
            }
            0xC5 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                3
            }
            0xCD => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                4
            }

            // CPX (Immediate / ZeroPage)
            0xE0 => {
                let v = self.immediate(mem);
                self.compare(self.x, v);
                2
            }
            0xE4 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.x, v);
                3
            }

            // CPY (Immediate / ZeroPage)
            0xC0 => {
                let v = self.immediate(mem);
                self.compare(self.y, v);
                2
            }
            0xC4 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.y, v);
                3
            }

            // AND (Immediate / ZeroPage / Absolute)
            0x29 => {
                let v = self.immediate(mem);
                self.a &= v;
                self.set_z_n(self.a);
                2
            }
            0x25 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                3
            }
            0x2D => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                4
            }

            // ORA (Immediate / ZeroPage / Absolute)
            0x09 => {
                let v = self.immediate(mem);
                self.a |= v;
                self.set_z_n(self.a);
                2
            }
            0x05 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                3
            }
            0x0D => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                4
            }

            // EOR (Immediate / ZeroPage / Absolute)
            0x49 => {
                let v = self.immediate(mem);
                self.a ^= v;
                self.set_z_n(self.a);
                2
            }
            0x45 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                3
            }
            0x4D => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                4
            }

            // INC / DEC (ZeroPage / Absolute)
            0xE6 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a).wrapping_add(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                5
            }
            0xEE => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a).wrapping_add(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                6
            }
            0xC6 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a).wrapping_sub(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                5
            }
            0xCE => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a).wrapping_sub(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                6
            }

            // JMP (indirect) — opcode 0x6C
            0x6C => {
                let addr_ptr = self.absolute(mem);
                let lo = self.read_byte(mem, addr_ptr) as u16;
                // قاعدة 6502: إذا كان LSB = 0xFF، نقرأ MSB من نفس الصفحة
                let hi_addr = if lo == 0x00FF {
                    addr_ptr & 0xFF00
                } else {
                    addr_ptr + 1
                };
                let hi = self.read_byte(mem, hi_addr) as u16;
                self.pc = (hi << 8) | lo;
                5
            }
            // TAX, TAY, TXA, TYA, TSX, TXS
            0xAA => {
                self.x = self.a;
                self.set_z_n(self.x);
                2
            }
            0xA8 => {
                self.y = self.a;
                self.set_z_n(self.y);
                2
            }
            0x8A => {
                self.a = self.x;
                self.set_z_n(self.a);
                2
            }
            0x98 => {
                self.a = self.y;
                self.set_z_n(self.a);
                2
            }
            0xBA => {
                self.x = self.sp;
                self.set_z_n(self.x);
                2
            }
            0x9A => {
                self.sp = self.x;
                2
            }

            // PHA, PLA, PHP, PLP
            0x48 => {
                self.push_byte(mem, self.a);
                3
            }
            0x68 => {
                self.a = self.pull_byte(mem);
                self.set_z_n(self.a);
                4
            }
            0x08 => {
                // PHP يدفع الحالة مع ضبط B flag (bit 4)
                self.push_byte(mem, self.status | 0x10);
                3
            }
            0x28 => {
                self.status = self.pull_byte(mem);
                4
            }
            // ADC (Immediate / ZeroPage / ZeroPageX / Absolute / AbsoluteX / AbsoluteY / IndirectX / IndirectY)
            0x69 => {
                let v = self.immediate(mem);
                let cycles = self.adc(v);
                cycles
            }
            0x65 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                3
            }
            0x75 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                4
            }
            0x6D => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                4
            }
            0x7D => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                4
            }
            0x79 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                4
            }
            0x61 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                6
            }
            0x71 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.adc(v);
                5
            }

            // SBC (Immediate / ZeroPage / ZeroPageX / Absolute / AbsoluteX / AbsoluteY / IndirectX / IndirectY)
            0xE9 => {
                let v = self.immediate(mem);
                self.sbc(v);
                2
            }
            0xE5 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                3
            }
            0xF5 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                4
            }
            0xED => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                4
            }
            0xFD => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                4
            }
            0xF9 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                4
            }
            0xE1 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                6
            }
            0xF1 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.sbc(v);
                5
            }

            // BIT (ZeroPage / Absolute)
            0x24 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.bit(v);
                3
            }
            0x2C => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.bit(v);
                4
            }

            // Branches
            0x10 => {
                let off = self.fetch_relative(mem);
                if !self.get_flag(Self::N) {
                    self.pc = self.pc.wrapping_add(off as u16);
                    4
                } else {
                    3
                }
            }
            0x30 => {
                let off = self.fetch_relative(mem);
                if self.get_flag(Self::N) {
                    self.pc = self.pc.wrapping_add(off as u16);
                    4
                } else {
                    3
                }
            }
            0x50 => {
                let off = self.fetch_relative(mem);
                if !self.get_flag(Self::V) {
                    self.pc = self.pc.wrapping_add(off as u16);
                    4
                } else {
                    3
                }
            }
            0x70 => {
                let off = self.fetch_relative(mem);
                if self.get_flag(Self::V) {
                    self.pc = self.pc.wrapping_add(off as u16);
                    4
                } else {
                    3
                }
            }
            0x90 => {
                let off = self.fetch_relative(mem);
                if !self.get_flag(Self::C) {
                    self.pc = self.pc.wrapping_add(off as u16);
                    4
                } else {
                    3
                }
            }
            0xB0 => {
                let off = self.fetch_relative(mem);
                if self.get_flag(Self::C) {
                    self.pc = self.pc.wrapping_add(off as u16);
                    4
                } else {
                    3
                }
            }
            // LDX (Immediate / ZeroPage / ZeroPageY / Absolute / AbsoluteY)
            0xA2 => {
                let v = self.immediate(mem);
                self.x = v;
                self.set_z_n(v);
                2
            }
            0xA6 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.x = v;
                self.set_z_n(v);
                3
            }
            0xB6 => {
                let a = self.zero_page_y(mem);
                let v = self.read_byte(mem, a);
                self.x = v;
                self.set_z_n(v);
                4
            }
            0xAE => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.x = v;
                self.set_z_n(v);
                4
            }
            0xBE => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.x = v;
                self.set_z_n(v);
                4
            }

            // LDY (Immediate / ZeroPage / ZeroPageX / Absolute / AbsoluteX)
            0xA0 => {
                let v = self.immediate(mem);
                self.y = v;
                self.set_z_n(v);
                2
            }
            0xA4 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.y = v;
                self.set_z_n(v);
                3
            }
            0xB4 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.y = v;
                self.set_z_n(v);
                4
            }
            0xAC => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.y = v;
                self.set_z_n(v);
                4
            }
            0xBC => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.y = v;
                self.set_z_n(v);
                4
            }

            // STX (ZeroPage / ZeroPageY / Absolute)
            0x86 => {
                let a = self.zero_page(mem);
                self.write_byte(mem, a, self.x);
                3
            }
            0x96 => {
                let a = self.zero_page_y(mem);
                self.write_byte(mem, a, self.x);
                4
            }
            0x8E => {
                let a = self.absolute(mem);
                self.write_byte(mem, a, self.x);
                4
            }

            // STY (ZeroPage / ZeroPageX / Absolute)
            0x84 => {
                let a = self.zero_page(mem);
                self.write_byte(mem, a, self.y);
                3
            }
            0x94 => {
                let a = self.zero_page_x(mem);
                self.write_byte(mem, a, self.y);
                4
            }
            0x8C => {
                let a = self.absolute(mem);
                self.write_byte(mem, a, self.y);
                4
            }

            // INC / DEC (ZeroPageX / AbsoluteX)
            0xF6 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a).wrapping_add(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                6
            }
            0xFE => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a).wrapping_add(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                7
            }
            0xD6 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a).wrapping_sub(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                6
            }
            0xDE => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a).wrapping_sub(1);
                self.write_byte(mem, a, v);
                self.set_z_n(v);
                7
            }

            // ASL (Accumulator / ZeroPage / ZeroPageX / Absolute / AbsoluteX)
            0x0A => {
                self.set_flag(Self::C, self.a & 0x80 != 0);
                self.a <<= 1;
                self.set_z_n(self.a);
                2
            }
            0x06 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = v << 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                5
            }
            0x16 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = v << 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x0E => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = v << 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x1E => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = v << 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                7
            }

            // LSR (Accumulator / ZeroPage / ZeroPageX / Absolute / AbsoluteX)
            0x4A => {
                self.set_flag(Self::C, self.a & 1 != 0);
                self.a >>= 1;
                self.set_z_n(self.a);
                2
            }
            0x46 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 1 != 0);
                let r = v >> 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                5
            }
            0x56 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 1 != 0);
                let r = v >> 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x4E => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 1 != 0);
                let r = v >> 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x5E => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.set_flag(Self::C, v & 1 != 0);
                let r = v >> 1;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                7
            }

            // ROL (Accumulator / ZeroPage / ZeroPageX / Absolute / AbsoluteX)
            0x2A => {
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, self.a & 0x80 != 0);
                self.a = (self.a << 1) | c;
                self.set_z_n(self.a);
                2
            }
            0x26 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = (v << 1) | c;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                5
            }
            0x36 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = (v << 1) | c;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x2E => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = (v << 1) | c;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x3E => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 0x80 != 0);
                let r = (v << 1) | c;
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                7
            }

            // ROR (Accumulator / ZeroPage / ZeroPageX / Absolute / AbsoluteX)
            0x6A => {
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, self.a & 1 != 0);
                self.a = (self.a >> 1) | (c << 7);
                self.set_z_n(self.a);
                2
            }
            0x66 => {
                let a = self.zero_page(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 1 != 0);
                let r = (v >> 1) | (c << 7);
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                5
            }
            0x76 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 1 != 0);
                let r = (v >> 1) | (c << 7);
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x6E => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 1 != 0);
                let r = (v >> 1) | (c << 7);
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                6
            }
            0x7E => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                let c = self.get_flag(Self::C) as u8;
                self.set_flag(Self::C, v & 1 != 0);
                let r = (v >> 1) | (c << 7);
                self.write_byte(mem, a, r);
                self.set_z_n(r);
                7
            }

            // CLI / SEI / CLV / NOP / BRK / RTI
            0x58 => {
                self.set_flag(Self::I, false);
                2
            }
            0x78 => {
                self.set_flag(Self::I, true);
                2
            }
            0xB8 => {
                self.set_flag(Self::V, false);
                2
            }
            0xEA => 2,
            0x00 => 7, // BRK مبسط
            0x40 => {
                self.status = self.pull_byte(mem);
                self.pc = self.pull_word(mem);
                6
            }
            0xB9 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.a = v;
                self.set_z_n(v);
                4
            }
            0x99 => {
                let a = self.absolute_y(mem);
                self.write_byte(mem, a, self.a);
                5
            }

            0xD5 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                4
            }
            0xDD => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                4
            }
            0xD9 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                4
            }
            0xC1 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                6
            }
            0xD1 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.a, v);
                5
            }

            0xEC => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.x, v);
                4
            }
            0xCC => {
                let a = self.absolute(mem);
                let v = self.read_byte(mem, a);
                self.compare(self.y, v);
                4
            }

            0x35 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                4
            }
            0x3D => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                4
            }
            0x39 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                4
            }
            0x21 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                6
            }
            0x31 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.a &= v;
                self.set_z_n(self.a);
                5
            }

            0x15 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                4
            }
            0x1D => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                4
            }
            0x19 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                4
            }
            0x01 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                6
            }
            0x11 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.a |= v;
                self.set_z_n(self.a);
                5
            }

            0x55 => {
                let a = self.zero_page_x(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                4
            }
            0x5D => {
                let a = self.absolute_x(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                4
            }
            0x59 => {
                let a = self.absolute_y(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                4
            }
            0x41 => {
                let a = self.indirect_x(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                6
            }
            0x51 => {
                let a = self.indirect_y(mem);
                let v = self.read_byte(mem, a);
                self.a ^= v;
                self.set_z_n(self.a);
                5
            }

            0xD8 => {
                self.set_flag(Self::D, false);
                2
            }
            0xF8 => {
                self.set_flag(Self::D, true);
                2
            }
            _ => 1,
        }
    }

    fn push_word(&mut self, mem: &mut Memory, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.push_byte(mem, hi);
        self.push_byte(mem, lo);
    }

    fn push_byte(&mut self, mem: &mut Memory, value: u8) {
        let addr = 0x100 | self.sp as u16;
        self.write_byte(mem, addr, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull_word(&mut self, mem: &mut Memory) -> u16 {
        let lo = self.pull_byte(mem) as u16;
        let hi = self.pull_byte(mem) as u16;
        (hi << 8) | lo
    }

    fn pull_byte(&mut self, mem: &mut Memory) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x100 | self.sp as u16;
        self.read_byte(mem, addr)
    }
}
