use za_math::Color;

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
    fn gray_from_tia(&self, value: u8) -> u8 {
        (((value & 0x7F) as u32) * 255 / 127) as u8
    }

    fn render_scanline(&mut self, row: usize) {
        let bk = self.gray_from_tia(self.colubk);
        let pf = self.gray_from_tia(self.colupf);
        let base = row * FB_WIDTH * 4;
        for x in 0..FB_WIDTH {
            let v = if self.playfield_bit(x as u32) { pf } else { bk };
            let idx = base + x * 4;
            self.framebuffer[idx] = v;
            self.framebuffer[idx + 1] = v;
            self.framebuffer[idx + 2] = v;
            self.framebuffer[idx + 3] = 255;
        }
    }

    pub fn background_color(&self) -> Color {
        let v = self.gray_from_tia(self.colubk) as f32 / 255.0;
        Color::new(v, v, v, 1.0)
    }
}
