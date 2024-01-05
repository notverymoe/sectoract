// Copyright 2023 Natalie Baker // AGPLv3 //

use bytemuck::{Zeroable, Pod, ByteHash, ByteEq};

/// 1 Map Unit = 2.5cm / ~1 inch, 40 Map Units per Metre
pub const UNIT_WORLD_I: i16 = 40;
pub const UNIT_WORLD_F: f32 = UNIT_WORLD_I as f32;

// Texture scale should be 4x Unit_World (~160 TU per metre)

// //

#[derive(Debug, Default, Clone, Copy, Zeroable, Pod, ByteHash, ByteEq)]
#[repr(C)]
#[repr(packed)]
pub struct SectorPoint2 {
    pub x: i16,
    pub y: i16,
}

impl SectorPoint2 {

    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self{x, y}
    }

    #[must_use]
    pub fn from_world(x: f32, y: f32) -> Self {
        Self { 
            x: from_world_pos(x), 
            y: from_world_pos(y) 
        }
    }

    #[must_use]
    pub fn to_world(&self) -> [f32; 2] {
        [
            to_world_pos(self.x),
            to_world_pos(self.y),
        ]
    }

    #[must_use]
    pub const fn extend(self, z: i16) -> SectorPoint3 {
        SectorPoint3::new(self.x, self.y, z)
    }

    #[must_use]
    pub const fn const_eq(self, other: Self) -> bool {
        constmuck::cast::<SectorPoint2, u32>(self) == constmuck::cast::<SectorPoint2, u32>(other)
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

    #[must_use]
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self{x, y, z}
    }

    #[must_use]
    pub fn to_world(&self) -> [f32; 3] {
        [
            to_world_pos(self.x),
            to_world_pos(self.y),
            to_world_pos(self.z),
        ]
    }

    #[must_use]
    pub const fn truncate(self) -> SectorPoint2 {
        SectorPoint2::new(self.x, self.y)
    }
}

#[must_use]
pub fn to_world_pos(v: i16) -> f32 {
    (v as f32)/UNIT_WORLD_F
}

#[must_use]
pub fn from_world_pos(v: f32) -> i16 {
    (v * UNIT_WORLD_F) as i16
}