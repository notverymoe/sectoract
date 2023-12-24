// Copyright 2023 Natalie Baker // AGPLv3 //

pub const UNIT_WORLD: f32 =   8.0;

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