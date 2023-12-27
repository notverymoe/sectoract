// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{IdentifierMaterial, SectorPoint2};

#[derive(Debug, Clone)]
pub struct SectorEdges {
    pub shape:     Vec<SectorPoint2>,
    pub materials: Vec<SectorEdgeMaterialSet>,
}

#[derive(Debug, Clone)]
pub struct SectorEdgeMaterialSet(Vec<SectorEdgeMaterialMasked>);

#[derive(Debug, Clone, Copy)]
pub struct SectorEdgeMaterialMasked {
    pub mask: [i16; 2],
    pub material: IdentifierMaterial,
}