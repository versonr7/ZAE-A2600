//! ذاكرة Atari 2600 الأساسية:
//! - ROM من 0x1000 إلى 0x1FFF (عادة 4K)
//! - RAM من 0x80 إلى 0xFF (128 بايت)
//! - سجلات TIA من 0x00 إلى 0x3F
//! - سجلات RIOT من 0x40 إلى 0x7F

use crate::tia::Tia;

pub const RAM_SIZE: usize = 128;
pub const ROM_SIZE: usize = 4096;

pub struct Memory {
    pub ram: [u8; RAM_SIZE],
    pub rom: [u8; ROM_SIZE],
    pub tia: Tia,
    pub riot_regs: [u8; 0x40],
}

impl Memory {
    pub const fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            rom: [0; ROM_SIZE],
            tia: Tia::new(),
            riot_regs: [0; 0x40],
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let len = data.len().min(ROM_SIZE);
        self.rom[..len].copy_from_slice(&data[..len]);
    }

    pub fn read(&self, addr: u16) -> u8 {
        let a = addr & 0x1FFF;
        match a {
            0x0000..=0x003F => self.tia.read_register(a & 0x3F),
            0x0040..=0x007F => self.riot_regs[(a & 0x3F) as usize],
            0x0080..=0x00FF => self.ram[(a & 0x7F) as usize],
            0x1000..=0x1FFF => self.rom[(a & 0x0FFF) as usize],
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let a = addr & 0x1FFF;
        match a {
            0x0000..=0x003F => self.tia.write_register(a & 0x3F, value),
            0x0040..=0x007F => self.riot_regs[(a & 0x3F) as usize] = value,
            0x0080..=0x00FF => self.ram[(a & 0x7F) as usize] = value,
            _ => {}
        }
    }
}
