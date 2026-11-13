use za_math::Color;

pub const NTSC_PALETTE: [(u8, u8, u8); 128] = [
    (0x00, 0x00, 0x00), (0x40, 0x40, 0x40), (0x6C, 0x6C, 0x6C), (0x90, 0x90, 0x90),
    (0xB0, 0xB0, 0xB0), (0xC8, 0xC8, 0xC8), (0xDC, 0xDC, 0xDC), (0xEC, 0xEC, 0xEC),
    (0x44, 0x44, 0x00), (0x64, 0x64, 0x10), (0x84, 0x84, 0x24), (0xA0, 0xA0, 0x34),
    (0xB8, 0xB8, 0x40), (0xD0, 0xD0, 0x50), (0xE8, 0xE8, 0x5C), (0xFC, 0xFC, 0x68),
    (0x70, 0x28, 0x00), (0x84, 0x44, 0x14), (0x98, 0x5C, 0x28), (0xAC, 0x74, 0x34),
    (0xBC, 0x88, 0x40), (0xCC, 0xA0, 0x4C), (0xDC, 0xB4, 0x58), (0xEC, 0xC8, 0x68),
    (0x84, 0x18, 0x00), (0x98, 0x34, 0x18), (0xAC, 0x50, 0x30), (0xC0, 0x68, 0x48),
    (0xD0, 0x80, 0x5C), (0xE0, 0x94, 0x70), (0xEC, 0xA8, 0x80), (0xFC, 0xBC, 0x94),
    (0x88, 0x00, 0x00), (0x9C, 0x20, 0x20), (0xB0, 0x3C, 0x3C), (0xC0, 0x58, 0x58),
    (0xD0, 0x70, 0x70), (0xE0, 0x88, 0x88), (0xEC, 0xA0, 0xA0), (0xFC, 0xB4, 0xB4),
    (0x78, 0x00, 0x5C), (0x8C, 0x20, 0x74), (0xA0, 0x3C, 0x88), (0xB0, 0x58, 0x9C),
    (0xC0, 0x70, 0xB0), (0xD0, 0x84, 0xC0), (0xDC, 0x9C, 0xD0), (0xEC, 0xB0, 0xE0),
    (0x48, 0x00, 0x78), (0x60, 0x20, 0x90), (0x78, 0x3C, 0xA4), (0x8C, 0x58, 0xB8),
    (0xA0, 0x70, 0xCC), (0xB4, 0x84, 0xDC), (0xC4, 0x9C, 0xEC), (0xD4, 0xB0, 0xFC),
    (0x14, 0x00, 0x84), (0x30, 0x20, 0x98), (0x4C, 0x3C, 0xAC), (0x68, 0x58, 0xC0),
    (0x7C, 0x70, 0xD0), (0x94, 0x88, 0xE0), (0xA8, 0xA0, 0xEC), (0xBC, 0xB4, 0xFC),
    (0x00, 0x00, 0x88), (0x1C, 0x20, 0x9C), (0x38, 0x40, 0xB0), (0x50, 0x5C, 0xC0),
    (0x68, 0x74, 0xD0), (0x7C, 0x8C, 0xE0), (0x90, 0xA4, 0xEC), (0xA4, 0xB8, 0xFC),
    (0x00, 0x18, 0x7C), (0x1C, 0x38, 0x90), (0x38, 0x54, 0xA8), (0x50, 0x70, 0xBC),
    (0x68, 0x88, 0xCC), (0x7C, 0x9C, 0xDC), (0x90, 0xB4, 0xEC), (0xA4, 0xC8, 0xFC),
    (0x00, 0x2C, 0x5C), (0x1C, 0x4C, 0x78), (0x38, 0x68, 0x90), (0x50, 0x84, 0xAC),
    (0x68, 0x9C, 0xC0), (0x7C, 0xB4, 0xD4), (0x90, 0xCC, 0xE8), (0xA4, 0xE0, 0xFC),
    (0x00, 0x3C, 0x2C), (0x1C, 0x5C, 0x48), (0x38, 0x7C, 0x64), (0x50, 0x9C, 0x80),
    (0x68, 0xB4, 0x94), (0x7C, 0xD0, 0xAC), (0x90, 0xE4, 0xC0), (0xA4, 0xFC, 0xD4),
    (0x00, 0x3C, 0x00), (0x20, 0x5C, 0x20), (0x40, 0x7C, 0x40), (0x5C, 0x9C, 0x5C),
    (0x74, 0xB4, 0x74), (0x8C, 0xD0, 0x8C), (0xA4, 0xE4, 0xA4), (0xB8, 0xFC, 0xB8),
    (0x14, 0x38, 0x00), (0x34, 0x5C, 0x1C), (0x50, 0x7C, 0x38), (0x6C, 0x98, 0x50),
    (0x84, 0xB4, 0x68), (0x9C, 0xCC, 0x7C), (0xB4, 0xE4, 0x94), (0xC8, 0xFC, 0xA8),
    (0x2C, 0x30, 0x00), (0x4C, 0x50, 0x1C), (0x68, 0x70, 0x34), (0x84, 0x8C, 0x4C),
    (0x9C, 0xA8, 0x64), (0xB4, 0xC0, 0x78), (0xCC, 0xD4, 0x88), (0xE0, 0xEC, 0x9C),
    (0x44, 0x28, 0x00), (0x64, 0x48, 0x18), (0x84, 0x68, 0x30), (0xA0, 0x84, 0x44),
    (0xB8, 0x9C, 0x58), (0xD0, 0xB4, 0x6C), (0xE8, 0xCC, 0x7C), (0xFC, 0xE0, 0x8C),
];

pub const SCANLINE_CYCLES: u32 = 228;
pub const TOTAL_SCANLINES: u16 = 262;
pub const VISIBLE_START: u16 = 40;
pub const FB_WIDTH: usize = 160;
pub const FB_HEIGHT: usize = 192;
pub const FB_SIZE: usize = FB_WIDTH * FB_HEIGHT * 4;

fn hpos_from_cycle(tia_cycle: u32) -> u8 {
    if tia_cycle < 68 {
        0
    } else {
        let px = tia_cycle - 68;
        if px > 159 { 159 } else { px as u8 }
    }
}

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
    pub colup0: u8,
    pub colup1: u8,
    pub grp0: u8,
    pub grp1: u8,
    pub hpos_p0: u8,
    pub hpos_p1: u8,
    pub hpos_m0: u8,
    pub hpos_m1: u8,
    pub hpos_bl: u8,
    pub nusiz0: u8,
    pub nusiz1: u8,
    pub enabl: bool,
    pub enam0: bool,
    pub enam1: bool,
    pub refp0: bool,
    pub refp1: bool,
    pub hmove_pending: i8,
    pub inpt4: u8,
    pub cxm0p: u8,
    pub cxm1p: u8,
    pub cxp0fb: u8,
    pub cxp1fb: u8,
    pub cxm0fb: u8,
    pub cxm1fb: u8,
    pub cxblpf: u8,
    pub cxppmm: u8,
    // ⚠️ [Cycle-Accurate] حقول HMOVE
    pub hmove_delay: u8,
    pub hmove_latch: bool,
}

impl Tia {
    pub const fn new() -> Self {
        Self {
            scanline: 0, colubk: 0, colupf: 0, ctrlpf: 0, pf0: 0, pf1: 0, pf2: 0,
            vblank: false, vsync: false, cycle_in_scanline: 0, wsync: false,
            framebuffer: [0; FB_SIZE], colup0: 0, colup1: 0, grp0: 0, grp1: 0,
            hpos_p0: 0, hpos_p1: 0, hpos_m0: 0, hpos_m1: 0, hpos_bl: 0,
            nusiz0: 0, nusiz1: 0, enabl: false, enam0: false, enam1: false,
            refp0: false, refp1: false, hmove_pending: 0, inpt4: 0x80,
            cxm0p: 0, cxm1p: 0, cxp0fb: 0, cxp1fb: 0, cxm0fb: 0, cxm1fb: 0,
            cxblpf: 0, cxppmm: 0, hmove_delay: 0, hmove_latch: false,
        }
    }

    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0x00 => { if self.vblank { 0x80 } else { 0x00 } }
            0x09 => self.colubk,
            0x30 => self.cxm0p, 0x31 => self.cxm1p, 0x32 => self.cxp0fb,
            0x33 => self.cxp1fb, 0x34 => self.cxm0fb, 0x35 => self.cxm1fb,
            0x36 => self.cxblpf, 0x37 => self.cxppmm, 0x3C => self.inpt4,
            0x3D => 0x80, _ => 0,
        }
    }

    pub fn clear_collision(&mut self, addr: u16) {
        match addr {
            0x30 => self.cxm0p = 0, 0x31 => self.cxm1p = 0, 0x32 => self.cxp0fb = 0,
            0x33 => self.cxp1fb = 0, 0x34 => self.cxm0fb = 0, 0x35 => self.cxm1fb = 0,
            0x36 => self.cxblpf = 0, 0x37 => self.cxppmm = 0, _ => {}
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => {
                let was = self.vsync;
                self.vsync = value & 0x02 != 0;
                if self.vsync && !was { self.scanline = 0; self.cycle_in_scanline = 0; self.hmove_latch = false; }
            }
            0x01 => self.vblank = value & 0x02 != 0,
            0x02 => self.wsync = true,
            0x04 => self.nusiz0 = value, 0x05 => self.nusiz1 = value,
            0x06 => self.colup0 = value, 0x07 => self.colup1 = value,
            0x08 => self.colupf = value, 0x09 => self.colubk = value,
            0x0A => self.ctrlpf = value,
            0x0B => self.refp0 = value & 0x08 != 0, 0x0C => self.refp1 = value & 0x08 != 0,
            0x0D => self.pf0 = value, 0x0E => self.pf1 = value, 0x0F => self.pf2 = value,
            0x10 => self.hpos_p0 = hpos_from_cycle(self.cycle_in_scanline),
            0x11 => self.hpos_p1 = hpos_from_cycle(self.cycle_in_scanline),
            0x12 => self.hpos_m0 = hpos_from_cycle(self.cycle_in_scanline),
            0x13 => self.hpos_m1 = hpos_from_cycle(self.cycle_in_scanline),
            0x14 => self.hpos_bl = hpos_from_cycle(self.cycle_in_scanline),
            0x1B => self.grp0 = value, 0x1C => self.grp1 = value,
            0x1D => self.enam0 = value & 0x02 != 0, 0x1E => self.enam1 = value & 0x02 != 0,
            0x1F => self.enabl = value & 0x02 != 0,
            0x20 | 0x21 | 0x22 | 0x23 | 0x24 => {
                let nibble = (value >> 4) & 0x0F;
                self.hmove_pending = if nibble < 8 { nibble as i8 } else if nibble == 8 { 0 } else { nibble as i8 - 16 };
            }
            0x2A => {
                self.hmove_latch = true;
                self.hmove_delay = 6;
            }
            0x2B => {
                self.hmove_pending = 0;
                self.hmove_delay = 0;
                self.hmove_latch = false;
            }
            _ => {}
        }
    }

    /// ⚠️ [Cycle-Accurate] تتقدم TIA بعدد من دورات TIA.
    pub fn tick_cycle(&mut self, tia_cycles: u32) {
        for _ in 0..tia_cycles {
            if self.hmove_delay > 0 {
                self.hmove_delay -= 1;
                if self.hmove_delay == 0 {
                    let d = self.hmove_pending as i16;
                    self.hpos_p0 = ((self.hpos_p0 as i16 + d).clamp(0, 159)) as u8;
                    self.hpos_p1 = ((self.hpos_p1 as i16 + d).clamp(0, 159)) as u8;
                    self.hpos_m0 = ((self.hpos_m0 as i16 + d).clamp(0, 159)) as u8;
                    self.hpos_m1 = ((self.hpos_m1 as i16 + d).clamp(0, 159)) as u8;
                    self.hpos_bl = ((self.hpos_bl as i16 + d).clamp(0, 159)) as u8;
                    self.hmove_pending = 0;
                    self.hmove_latch = false;
                }
            }

            self.cycle_in_scanline += 1;
            if self.cycle_in_scanline >= SCANLINE_CYCLES {
                self.cycle_in_scanline = 0;
                self.hmove_latch = false;
                self.end_of_scanline();
            }
        }
    }

    fn end_of_scanline(&mut self) {
        if self.scanline >= VISIBLE_START && (self.scanline - VISIBLE_START) < FB_HEIGHT as u16 {
            let row = (self.scanline - VISIBLE_START) as usize;
            if self.vblank {
                let base = row * FB_WIDTH * 4;
                for i in 0..(FB_WIDTH * 4) { self.framebuffer[base + i] = 0; }
            } else {
                self.render_scanline(row);
            }
        }
        self.scanline = self.scanline.wrapping_add(1);
        if self.scanline >= TOTAL_SCANLINES { self.scanline = 0; }
    }

    fn playfield_bit(&self, x: u32) -> bool {
        let is_right_half = x >= 80;
        let reflect = (self.ctrlpf & 0x01) != 0;
        let col = if is_right_half && reflect { 159 - x } else if is_right_half { x - 80 } else { x };
        let bit_index = col / 4;
        match bit_index {
            0..=3 => { let bit_to_check = 3 - bit_index; (self.pf0 >> (4 + bit_to_check)) & 1 != 0 }
            4..=11 => { let bit_to_check = bit_index - 4; (self.pf1 >> bit_to_check) & 1 != 0 }
            12..=19 => { let bit_to_check = 7 - (bit_index - 12); (self.pf2 >> bit_to_check) & 1 != 0 }
            _ => false,
        }
    }

    fn get_color(&self, value: u8) -> (u8, u8, u8) {
        let hue = (value >> 4) & 0x0F;
        let lum = (value >> 1) & 0x07;
        NTSC_PALETTE[(hue as usize) * 8 + (lum as usize)]
    }

    fn render_scanline(&mut self, row: usize) {
        if self.vblank { return; }
        let (br, bg, bb) = self.get_color(self.colubk);
        let (pr, pg, pb) = self.get_color(self.colupf);
        let (p0r, p0g, p0b) = self.get_color(self.colup0);
        let (p1r, p1g, p1b) = self.get_color(self.colup1);
        let base = row * FB_WIDTH * 4;
        let score_mode = (self.ctrlpf & 0x02) != 0;

        for x in 0..FB_WIDTH {
            let mut r = br; let mut g = bg; let mut b = bb;
            let is_pf = self.playfield_bit(x as u32);
            if is_pf {
                if score_mode {
                    if x < FB_WIDTH / 2 { let (c0r, c0g, c0b) = self.get_color(self.colup0); r = c0r; g = c0g; b = c0b; }
                    else { let (c1r, c1g, c1b) = self.get_color(self.colup1); r = c1r; g = c1g; b = c1b; }
                } else { r = pr; g = pg; b = pb; }
            }
            let mut is_p0 = false;
            if self.grp0 != 0 {
                let dx = x as i32 - self.hpos_p0 as i32;
                if (0..8).contains(&dx) {
                    let bit = if self.refp0 { dx } else { 7 - dx };
                    if (self.grp0 >> bit) & 1 != 0 { r = p0r; g = p0g; b = p0b; is_p0 = true; }
                }
            }
            let mut is_p1 = false;
            if self.grp1 != 0 {
                let dx = x as i32 - self.hpos_p1 as i32;
                if (0..8).contains(&dx) {
                    let bit = if self.refp1 { dx } else { 7 - dx };
                    if (self.grp1 >> bit) & 1 != 0 { r = p1r; g = p1g; b = p1b; is_p1 = true; }
                }
            }
            let mut is_bl = false;
            if self.enabl {
                let dx = x as i32 - self.hpos_bl as i32;
                if (0..2).contains(&dx) { r = pr; g = pg; b = pb; is_bl = true; }
            }
            let mut is_m0 = false;
            if self.enam0 {
                let dx = x as i32 - self.hpos_m0 as i32;
                if (0..2).contains(&dx) { r = p0r; g = p0g; b = p0b; is_m0 = true; }
            }
            let mut is_m1 = false;
            if self.enam1 {
                let dx = x as i32 - self.hpos_m1 as i32;
                if (0..2).contains(&dx) { r = p1r; g = p1g; b = p1b; is_m1 = true; }
            }
            if is_p0 && is_p1 { self.cxppmm |= 0x80; }
            if is_p0 && is_pf { self.cxp0fb |= 0x80; }
            if is_p0 && is_bl { self.cxp0fb |= 0x40; }
            if is_p1 && is_pf { self.cxp1fb |= 0x80; }
            if is_p1 && is_bl { self.cxp1fb |= 0x40; }
            if is_m0 && is_m1 { self.cxppmm |= 0x40; }
            if is_m0 && is_p1 { self.cxm0p |= 0x80; }
            if is_m1 && is_p0 { self.cxm1p |= 0x80; }
            if is_bl && is_pf { self.cxblpf |= 0x80; }

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