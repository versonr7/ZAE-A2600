use za_math::Color;

// ألوان NTSC الحقيقية لـ Atari 2600 (128 لوناً، 16 صف × 8 ألوان)
pub const NTSC_PALETTE: [(u8, u8, u8); 128] = [
    // Column 0: Gray
    (0x00, 0x00, 0x00),
    (0x40, 0x40, 0x40),
    (0x6C, 0x6C, 0x6C),
    (0x90, 0x90, 0x90),
    (0xB0, 0xB0, 0xB0),
    (0xC8, 0xC8, 0xC8),
    (0xDC, 0xDC, 0xDC),
    (0xEC, 0xEC, 0xEC),
    // Column 1: Gold
    (0x44, 0x44, 0x00),
    (0x64, 0x64, 0x10),
    (0x84, 0x84, 0x24),
    (0xA0, 0xA0, 0x34),
    (0xB8, 0xB8, 0x40),
    (0xD0, 0xD0, 0x50),
    (0xE8, 0xE8, 0x5C),
    (0xFC, 0xFC, 0x68),
    // Column 2: Orange
    (0x70, 0x28, 0x00),
    (0x84, 0x44, 0x14),
    (0x98, 0x5C, 0x28),
    (0xAC, 0x74, 0x34),
    (0xBC, 0x88, 0x40),
    (0xCC, 0xA0, 0x4C),
    (0xDC, 0xB4, 0x58),
    (0xEC, 0xC8, 0x68),
    // Column 3: Orange-Red
    (0x84, 0x18, 0x00),
    (0x98, 0x34, 0x18),
    (0xAC, 0x50, 0x30),
    (0xC0, 0x68, 0x48),
    (0xD0, 0x80, 0x5C),
    (0xE0, 0x94, 0x70),
    (0xEC, 0xA8, 0x80),
    (0xFC, 0xBC, 0x94),
    // Column 4: Red
    (0x88, 0x00, 0x00),
    (0x9C, 0x20, 0x20),
    (0xB0, 0x3C, 0x3C),
    (0xC0, 0x58, 0x58),
    (0xD0, 0x70, 0x70),
    (0xE0, 0x88, 0x88),
    (0xEC, 0xA0, 0xA0),
    (0xFC, 0xB4, 0xB4),
    // Column 5: Pink
    (0x78, 0x00, 0x5C),
    (0x8C, 0x20, 0x74),
    (0xA0, 0x3C, 0x88),
    (0xB0, 0x58, 0x9C),
    (0xC0, 0x70, 0xB0),
    (0xD0, 0x84, 0xC0),
    (0xDC, 0x9C, 0xD0),
    (0xEC, 0xB0, 0xE0),
    // Column 6: Purple
    (0x48, 0x00, 0x78),
    (0x60, 0x20, 0x90),
    (0x78, 0x3C, 0xA4),
    (0x8C, 0x58, 0xB8),
    (0xA0, 0x70, 0xCC),
    (0xB4, 0x84, 0xDC),
    (0xC4, 0x9C, 0xEC),
    (0xD4, 0xB0, 0xFC),
    // Column 7: Purple-Blue
    (0x14, 0x00, 0x84),
    (0x30, 0x20, 0x98),
    (0x4C, 0x3C, 0xAC),
    (0x68, 0x58, 0xC0),
    (0x7C, 0x70, 0xD0),
    (0x94, 0x88, 0xE0),
    (0xA8, 0xA0, 0xEC),
    (0xBC, 0xB4, 0xFC),
    // Column 8: Blue
    (0x00, 0x00, 0x88),
    (0x1C, 0x20, 0x9C),
    (0x38, 0x40, 0xB0),
    (0x50, 0x5C, 0xC0),
    (0x68, 0x74, 0xD0),
    (0x7C, 0x8C, 0xE0),
    (0x90, 0xA4, 0xEC),
    (0xA4, 0xB8, 0xFC),
    // Column 9: Light Blue
    (0x00, 0x18, 0x7C),
    (0x1C, 0x38, 0x90),
    (0x38, 0x54, 0xA8),
    (0x50, 0x70, 0xBC),
    (0x68, 0x88, 0xCC),
    (0x7C, 0x9C, 0xDC),
    (0x90, 0xB4, 0xEC),
    (0xA4, 0xC8, 0xFC),
    // Column 10: Cyan
    (0x00, 0x2C, 0x5C),
    (0x1C, 0x4C, 0x78),
    (0x38, 0x68, 0x90),
    (0x50, 0x84, 0xAC),
    (0x68, 0x9C, 0xC0),
    (0x7C, 0xB4, 0xD4),
    (0x90, 0xCC, 0xE8),
    (0xA4, 0xE0, 0xFC),
    // Column 11: Cyan-Green
    (0x00, 0x3C, 0x2C),
    (0x1C, 0x5C, 0x48),
    (0x38, 0x7C, 0x64),
    (0x50, 0x9C, 0x80),
    (0x68, 0xB4, 0x94),
    (0x7C, 0xD0, 0xAC),
    (0x90, 0xE4, 0xC0),
    (0xA4, 0xFC, 0xD4),
    // Column 12: Green
    (0x00, 0x3C, 0x00),
    (0x20, 0x5C, 0x20),
    (0x40, 0x7C, 0x40),
    (0x5C, 0x9C, 0x5C),
    (0x74, 0xB4, 0x74),
    (0x8C, 0xD0, 0x8C),
    (0xA4, 0xE4, 0xA4),
    (0xB8, 0xFC, 0xB8),
    // Column 13: Yellow-Green
    (0x14, 0x38, 0x00),
    (0x34, 0x5C, 0x1C),
    (0x50, 0x7C, 0x38),
    (0x6C, 0x98, 0x50),
    (0x84, 0xB4, 0x68),
    (0x9C, 0xCC, 0x7C),
    (0xB4, 0xE4, 0x94),
    (0xC8, 0xFC, 0xA8),
    // Column 14: Yellow
    (0x2C, 0x30, 0x00),
    (0x4C, 0x50, 0x1C),
    (0x68, 0x70, 0x34),
    (0x84, 0x8C, 0x4C),
    (0x9C, 0xA8, 0x64),
    (0xB4, 0xC0, 0x78),
    (0xCC, 0xD4, 0x88),
    (0xE0, 0xEC, 0x9C),
    // Column 15: Orange-Yellow
    (0x44, 0x28, 0x00),
    (0x64, 0x48, 0x18),
    (0x84, 0x68, 0x30),
    (0xA0, 0x84, 0x44),
    (0xB8, 0x9C, 0x58),
    (0xD0, 0xB4, 0x6C),
    (0xE8, 0xCC, 0x7C),
    (0xFC, 0xE0, 0x8C),
];

pub const TIA_VSYNC: u16 = 0x00;
pub const TIA_VBLANK: u16 = 0x01;
pub const TIA_WSYNC: u16 = 0x02;
pub const TIA_COLUPF: u16 = 0x08;
pub const TIA_COLUBK: u16 = 0x09;
pub const TIA_CTRLPF: u16 = 0x0A;
pub const TIA_PF0: u16 = 0x0D;
pub const TIA_PF1: u16 = 0x0E;
pub const TIA_PF2: u16 = 0x0F;

pub const SCANLINE_CYCLES: u32 = 76;
pub const TOTAL_SCANLINES: u16 = 262;
pub const VISIBLE_START: u16 = 40; // 3 VSYNC + 37 VBLANK (NTSC قياسي)
pub const FB_WIDTH: usize = 160;
pub const FB_HEIGHT: usize = 192;
pub const FB_SIZE: usize = FB_WIDTH * FB_HEIGHT * 4;

pub struct Tia {
    pub scanline: u16,
    pub colubk: u8,
    pub colupf: u8,
    pub ctrlpf: u8,
    pub pf0: u8,
    pub pf1: u8,
    pub pf2: u8,
    pub vblank: bool,
    pub vsync: bool,
    pub cycle_in_scanline: u32,
    pub wsync: bool,
    pub framebuffer: [u8; FB_SIZE],
}

impl Tia {
    pub const fn new() -> Self {
        Self {
            scanline: 0,
            colubk: 0,
            colupf: 0,
            ctrlpf: 0,
            pf0: 0,
            pf1: 0,
            pf2: 0,
            vblank: false,
            vsync: false,
            cycle_in_scanline: 0,
            wsync: false,
            framebuffer: [0; FB_SIZE],
        }
    }

    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            TIA_VBLANK => {
                if self.vblank {
                    0x80
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
            TIA_VSYNC => {
                let was = self.vsync;
                self.vsync = value & 0x02 != 0;
                // بداية نبضة VSYNC جديدة: صفّر عداد خط المسح يبقى متزامن كل إطار
                if self.vsync && !was {
                    self.scanline = 0;
                    self.cycle_in_scanline = 0;
                }
            }
            TIA_VBLANK => self.vblank = value & 0x02 != 0,
            TIA_WSYNC => self.wsync = true,
            TIA_COLUPF => self.colupf = value,
            TIA_COLUBK => self.colubk = value,
            TIA_CTRLPF => self.ctrlpf = value,
            TIA_PF0 => self.pf0 = value,
            TIA_PF1 => self.pf1 = value,
            TIA_PF2 => self.pf2 = value,
            _ => {}
        }
    }

    pub fn tick(&mut self, cycles: u32) {
        let mut remaining = cycles;
        while remaining > 0 {
            let space_left = SCANLINE_CYCLES - self.cycle_in_scanline;
            let consume = remaining.min(space_left);
            self.cycle_in_scanline += consume;
            remaining -= consume;
            if self.cycle_in_scanline >= SCANLINE_CYCLES {
                self.cycle_in_scanline = 0;
                self.end_of_scanline();
            }
        }
    }

    fn end_of_scanline(&mut self) {
        if self.scanline >= VISIBLE_START && (self.scanline - VISIBLE_START) < FB_HEIGHT as u16 {
            let row = (self.scanline - VISIBLE_START) as usize;
            self.render_scanline(row);
        }
        self.scanline = self.scanline.wrapping_add(1);
        if self.scanline >= TOTAL_SCANLINES {
            self.scanline = 0;
        }
    }

    // موقع البكسل x (0..160): جزء من الـ playfield ("1") أو الخلفية ("0")؟
    // مطابق لترتيب PF0/PF1/PF2 الرسمي بمواصفات TIA (تحققت منه)
    fn playfield_bit(&self, x: u32) -> bool {
        let col = if x < 80 {
            x
        } else if self.ctrlpf & 0x01 != 0 {
            159 - x // معكوس (reflected)
        } else {
            x - 80 // مكرر (repeated)
        };
        let bit_index = col / 4; // 0..19
        match bit_index {
            0..=3 => (self.pf0 >> (4 + bit_index)) & 1 != 0,
            4..=11 => (self.pf1 >> (11 - bit_index)) & 1 != 0,
            12..=19 => (self.pf2 >> (bit_index - 12)) & 1 != 0,
            _ => false,
        }
    }

    // تحويل مبسّط (رمادي وليس ألوان NTSC حقيقية بعد) — يكفي حالياً
    // نميّز فيه الـ playfield عن الخلفية بوضوح. نحسّنه لاحقاً.
    fn get_color(&self, value: u8) -> (u8, u8, u8) {
        // bit 7 (التوهج) يُتجاهل في NTSC
        NTSC_PALETTE[(value & 0x7F) as usize]
    }

    fn render_scanline(&mut self, row: usize) {
        let (br, bg, bb) = self.get_color(self.colubk);
        let (pr, pg, pb) = self.get_color(self.colupf);
        let base = row * FB_WIDTH * 4;
        for x in 0..FB_WIDTH {
            let (r, g, b) = if self.playfield_bit(x as u32) {
                (pr, pg, pb)
            } else {
                (br, bg, bb)
            };
            let idx = base + x * 4;
            self.framebuffer[idx] = r;
            self.framebuffer[idx + 1] = g;
            self.framebuffer[idx + 2] = b;
            self.framebuffer[idx + 3] = 255;
        }
    }

    pub fn background_color(&self) -> Color {
        let (r, g, b) = self.get_color(self.colubk);
        Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }
}
