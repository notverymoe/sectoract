// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{IdentifierEdgeHalf, IdentifierSection, Section, Point2};

#[derive(Debug)]
pub struct Sector {
    pub graph:    HashMap<IdentifierEdgeHalf, EdgeHalf>,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeHalf {
    pub next:    Point2,
    pub section: IdentifierSection,
}

impl EdgeHalf {
    
    #[must_use]
    pub const fn new(next: Point2, section: IdentifierSection) -> Self {
        Self{next, section}
    }

}