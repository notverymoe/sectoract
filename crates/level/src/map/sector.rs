// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{
    map::{SectorPoint2, SectorPoint3},
    graphics::TextureFace
};

pub struct Sector {
    pub edge_points:  Box<[SectorPoint2]>,
    pub edge_texture: Box<[TextureFace ]>,

    pub surface_upper: SectorSurface,
    pub surface_lower: SectorSurface,
}

pub struct SectorAnchor(u16);

pub struct SectorSurface {
    pub texture: TextureFace,
    pub slope:   SectorSlope,
}

pub struct SectorSlope {
    pub anchor: SectorAnchor,
    pub start: i32,
    pub end:   i32,
}

// //

pub struct SectorConnection {
    pub idx_sector_outer: u16,
    pub idx_sector_inner: u16,
    pub position: SectorPoint3,
}
