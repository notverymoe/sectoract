// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::graphics::{MaterialTexture, IdentifierTexture};

mod angle;
pub use angle::*;

mod point;
pub use point::*;

mod sector;
pub use sector::*;

mod transform;
pub use transform::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierMaterial(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierSector(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierEdge {
    idx_sector: IdentifierSector,
    idx_edge:   u16,
}

pub struct Map {
    pub sectors:     Vec<Sector>,
    pub nestings:    Vec<SectorNested>,
    pub connections: Vec<EdgeConnection>,
    pub materials:   Vec<MaterialTexture>,
    pub textures:    HashMap<IdentifierTexture, String>,
}
