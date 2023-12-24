// Copyright 2023 Natalie Baker // AGPLv3 //

pub const UNIT_TEXTURE: f32 = 256.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureOffset {
    pub x: i16,
    pub y: i16,
}

impl TextureOffset {
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self{x, y}
    }

    #[must_use]
    pub fn to_world(&self) -> [f32; 2] {
        [to_world(self.x), to_world(self.y)]
    }
}

#[must_use]
pub fn to_world(v: i16) -> f32 {
    (v as f32)/UNIT_TEXTURE
}

#[must_use]
pub fn from_world(v: f32) -> i16 {
    (v * UNIT_TEXTURE) as i16
}
