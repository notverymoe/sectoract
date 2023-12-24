// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{graphics::TextureFace, map::SectorPoint2};

mod slope;
pub use slope::*;

mod connection_stack;
pub use connection_stack::*;

pub struct Sector {
    pub edge_points:  Box<[SectorPoint2]>,
    pub edge_texture: Box<[TextureFace ]>,

    pub surface_upper: SectorSurface,
    pub surface_lower: SectorSurface,
}

pub struct SectorSurface {
    pub texture: TextureFace,
    pub slope:   SectorSlope,
}