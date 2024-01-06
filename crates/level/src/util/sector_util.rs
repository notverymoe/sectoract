// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{IdentifierSection, IdentifierEdgeHalf, EdgeHalf, Point2};

use super::SectionIter;

#[must_use]
pub fn extract_section_points_from_sector<S: core::hash::BuildHasher>(graph: &HashMap<IdentifierEdgeHalf, EdgeHalf, S>, section: IdentifierSection) -> Option<Vec<Point2>> {
    for (k, v) in graph {
        if v.section != section { continue; }
        return Some(SectionIter::new(graph, *k).map(|(e, _h)| e.prev()).collect());
    }
    None
}