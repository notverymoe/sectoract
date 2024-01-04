// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{IdentifierEdgeHalf, SectorPoint2, Section, IdentifierSectorPoint, IdentifierSection};

pub struct Sector {
    pub points:   Vec<SectorPoint2>,
    pub graph:    HashMap<IdentifierEdgeHalf, EdgeHalf>,
    pub sections: Vec<Section>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeHalf {
    pub next:    IdentifierSectorPoint,
    pub section: IdentifierSection,
}