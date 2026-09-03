use za_math::Color;

pub const TIA_VSYNC: u16 = 0x00;
pub const TIA_VBLANK: u16 = 0x01;
pub const TIA_WSYNC: u16 = 0x02;
pub const TIA_COLUBK: u16 = 0x09;

pub struct Tia {
    pub scanline: u16,
    pub colubk: u8,
    pub vblank: bool,
    pub vsync: bool,
    pub cycle_in_scanline: u32,
    pub wsync: bool,
}

impl Tia {
    pub const fn new() -> Self {
        Self {
            scanline: 0,
            colubk: 0,
            vblank: false,
            vsync: false,
            cycle_in_scanline: 0,
            wsync: false,
        }
    }

    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            TIA_VBLANK => {
                if self.vblank {
                    0x02
                } else {
                    0x00
                }
            }
            TIA_COLUBK => self.colubk,
            _ => 0,
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => {
                self.vsync = value & 0x02 != 0;
            }
            0x01 => {
                self.vblank = value & 0x02 != 0;
            }
            0x02 => {
                self.wsync = true;
            }
            0x09 => {
                self.colubk = value;
            }
            _ => {}
        }
    }

    pub fn tick(&mut self, cycles: u32) {
        self.cycle_in_scanline += cycles;
        if self.cycle_in_scanline >= 76 {
            let lines = (self.cycle_in_scanline / 76) as u16;
            self.scanline = self.scanline.wrapping_add(lines);
            self.cycle_in_scanline %= 76;
            if self.scanline > 262 {
                self.scanline = 0;
            }
        }
    }

    pub fn background_color(&self) -> Color {
        let v = (self.colubk & 0x7F) as f32 / 127.0;
        Color::new(v, v, v, 1.0)
    }
}
