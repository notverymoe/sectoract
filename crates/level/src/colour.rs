// Copyright 2023 Natalie Baker // AGPLv3 //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColourRGBA(u16);

impl Default for ColourRGBA {
    fn default() -> Self {
        Self::WHITE
    }
}

impl ColourRGBA {

    pub const CHANNEL_MAX: u8 = 0x0F;

    pub const BLACK: ColourRGBA = ColourRGBA(0x000F);
    pub const WHITE: ColourRGBA = ColourRGBA(0xFFFF);

    pub const RED:   ColourRGBA = ColourRGBA(0xF00F);
    pub const GREEN: ColourRGBA = ColourRGBA(0x0F0F);
    pub const BLUE:  ColourRGBA = ColourRGBA(0x00FF);

    pub const YELLOW:  ColourRGBA = ColourRGBA(0xFF0F);
    pub const CYAN:    ColourRGBA = ColourRGBA(0x0FFF);
    pub const MAGENTA: ColourRGBA = ColourRGBA(0xF0FF);

    pub const TRANSPARENT: ColourRGBA = ColourRGBA(0x0000);
}

impl ColourRGBA {

    pub const fn new(v: u16) -> Self {
        Self(v)
    }

    pub const fn from_rgba8(v: u32) -> Self {
        Self::new(
            (((v & 0xF0000000) >> 12) as u16) |
            (((v & 0x00F00000) >>  8) as u16) |
            (((v & 0x0000F000) >>  4) as u16) |
             ((v & 0x000000F0)        as u16)
        )
    }

    pub const fn to_rgba8(self) -> u32 {
        (self.r() as u32) << 24 |
        (self.g() as u32) << 16 |
        (self.b() as u32) <<  8 |
        (self.a() as u32) 
    }

    pub fn from_rgba32(v: [f32; 4]) -> Self {
        Self::new(
            (((v[0] * 16.0) as u16) << 12) |
            (((v[1] * 16.0) as u16) <<  8) |
            (((v[2] * 16.0) as u16) <<  4) |
             ((v[3] * 16.0) as u16)
        )
    }

    pub fn to_rgba32(self) -> [f32; 4] {
        [
            (self.r() as f32)/16.0,
            (self.g() as f32)/16.0,
            (self.b() as f32)/16.0,
            (self.a() as f32)/16.0,
        ]
    }

}

impl ColourRGBA {

    pub const fn r(&self) -> u8 {
        ((self.0 >> 12) & 0x000F) as u8
    }

    pub const fn g(&self) -> u8 {
        ((self.0 >> 8) & 0x000F) as u8
    }

    pub const fn b(&self) -> u8 {
        ((self.0 >> 4) & 0x000F) as u8
    }

    pub const fn a(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

}

impl ColourRGBA {

    pub fn set_r(&mut self, r: u8) -> &mut Self {
        *self = self.with_r(r);
        self
    }

    pub fn set_g(&mut self, g: u8) -> &mut Self {
        *self = self.with_g(g);
        self
    }

    pub fn set_b(&mut self, b: u8) -> &mut Self {
        *self = self.with_b(b);
        self
    }

    pub fn set_a(&mut self, a: u8) -> &mut Self {
        *self = self.with_a(a);
        self
    }

    pub const fn with_r(self, r: u8) -> Self {
        Self::new((self.0 & 0x0FFF) | (((r & 0x0F) as u16) << 12))
    }

    pub const fn with_g(self, g: u8) -> Self {
        Self::new((self.0 & 0xF0FF) | (((g & 0x0F) as u16) << 8))
    }

    pub const fn with_b(self, b: u8) -> Self {
        Self::new((self.0 & 0xFF0F) | (((b & 0x0F) as u16) << 4))
    }

    pub const fn with_a(self, a: u8) -> Self {
        Self::new((self.0 & 0xFFF0) | ((a & 0x0F) as u16))
    }

}
