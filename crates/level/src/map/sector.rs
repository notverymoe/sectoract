// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{IdentifierEdgeHalf, IdentifierSection, Section, Point2};

pub struct Sector {
    pub graph:    HashMap<IdentifierEdgeHalf, EdgeHalf>,
    pub sections: Vec<Section>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeHalf {
    pub next:    Point2,
    pub section: IdentifierSection,
}