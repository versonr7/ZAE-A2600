//! ذاكرة Atari 2600 الأساسية

use crate::tia::Tia;

pub const RAM_SIZE: usize = 128;
pub const ROM_SIZE: usize = 4096;

pub struct Memory {
    pub ram: [u8; RAM_SIZE],
    pub rom: [u8; ROM_SIZE],
    pub tia: Tia,
    pub riot_regs: [u8; 0x40],
    pub riot_timer: u8,
    pub riot_prescaler: u32,
    pub riot_prescaler_value: u32,
}

impl Memory {
    pub const fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            rom: [0; ROM_SIZE],
            tia: Tia::new(),
            riot_regs: [0; 0x40],
            riot_timer: 0,
            riot_prescaler: 1,
            riot_prescaler_value: 1,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let len = data.len().min(ROM_SIZE);
        self.rom[..len].copy_from_slice(&data[..len]);
    }

    fn riot_read(&self, addr: u16) -> u8 {
        match addr & 0x1F {
            0x04 | 0x06 => self.riot_timer, // INTIM
            0x05 | 0x07 => {
                if self.riot_timer == 0 {
                    0x80
                } else {
                    0x00
                } // INSTAT
            }
            _ => 0,
        }
    }

    fn riot_write(&mut self, addr: u16, value: u8) {
        match addr & 0x1F {
            0x14 => {
                // TIM1T
                self.riot_timer = value;
                self.riot_prescaler_value = 1;
                self.riot_prescaler = 1;
            }
            0x15 => {
                // TIM8T
                self.riot_timer = value;
                self.riot_prescaler_value = 8;
                self.riot_prescaler = 8;
            }
            0x16 => {
                // TIM64T  ← الأهم!
                self.riot_timer = value;
                self.riot_prescaler_value = 64;
                self.riot_prescaler = 64;
            }
            0x17 => {
                // T1024T
                self.riot_timer = value;
                self.riot_prescaler_value = 1024;
                self.riot_prescaler = 1024;
            }
            0x02 => {
                // SWCHB
                self.riot_regs[2] = value;
            }
            _ => {}
        }
    }

    pub fn tick_riot(&mut self, cycles: u32) {
        for _ in 0..cycles {
            if self.riot_prescaler > 0 {
                self.riot_prescaler -= 1;
            }
            if self.riot_prescaler == 0 {
                self.riot_prescaler = self.riot_prescaler_value;
                self.riot_timer = self.riot_timer.wrapping_sub(1);
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let a = addr & 0x1FFF;
        match a {
            0x0000..=0x007F => self.tia.read_register(a & 0x3F),
            0x0080..=0x00FF => self.ram[(a & 0x7F) as usize],
            0x0180..=0x01FF => self.ram[(a & 0x7F) as usize],
            0x0280..=0x02FF => self.riot_read(a),
            0x1000..=0x1FFF => self.rom[(a & 0x0FFF) as usize],
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let a = addr & 0x1FFF;
        match a {
            0x0000..=0x007F => self.tia.write_register(a & 0x3F, value),
            0x0080..=0x00FF => self.ram[(a & 0x7F) as usize] = value,
            0x0180..=0x01FF => self.ram[(a & 0x7F) as usize] = value,
            0x0280..=0x02FF => self.riot_write(a, value),
            _ => {}
        }
    }
}
