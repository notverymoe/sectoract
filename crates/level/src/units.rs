// Copyright 2023 Natalie Baker // AGPLv3 //

pub const UNIT_WORLD:   f32 =   8.0;
pub const UNIT_TEXTURE: f32 = 256.0;

// //

#[derive(Debug, Default, Clone, Copy)]
pub struct SectorPoint2 {
    pub x: i16,
    pub y: i16,
}

impl SectorPoint2 {

    pub fn new(x: i16, y: i16) -> Self {
        Self{x, y}
    }

    pub fn to_world(&self) -> [f32; 3] {
        [
            to_world_pos(self.x),
            to_world_pos(self.y),
            0.0
        ]
    }

    pub fn extend(self, z: i16) -> SectorPoint3 {
        SectorPoint3::new(self.x, self.y, z)
    }
}

// ///

#[derive(Debug, Default, Clone, Copy)]
pub struct SectorPoint3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl SectorPoint3 {

    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self{x, y, z}
    }

    pub fn to_world(&self) -> [f32; 3] {
        [
            to_world_pos(self.x),
            to_world_pos(self.y),
            to_world_pos(self.z),
        ]
    }

    pub fn truncate(self) -> SectorPoint2 {
        SectorPoint2::new(self.x, self.y)
    }
}

pub fn to_world_pos(v: i16) -> f32 {
    (v as f32)/UNIT_WORLD
}

pub fn from_world_pos(v: f32) -> i16 {
    (v * UNIT_WORLD) as i16
}

pub fn to_texture_pos(v: i16) -> f32 {
    (v as f32)/UNIT_TEXTURE
}

pub fn from_texture_pos(v: f32) -> i16 {
    (v * UNIT_TEXTURE) as i16
}

pub fn to_world_angle(v: u16) -> f32 {
    std::f32::consts::TAU/(v as f32)
}

pub fn from_world_angle(v: f32) -> u16 {
    (std::f32::consts::TAU/v).rem_euclid(u16::MAX as f32) as u16
}