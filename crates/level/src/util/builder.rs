// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{Point2, Section, Sector, IdentifierEdgeHalf, EdgeHalf, IdentifierSection};

use super::extract_boundry_from_sector;

#[derive(Debug, Default)]
pub struct SectorBuilder {
    sections: Vec<Section>,
    edges:    Vec<Box<[Point2]>>,
}

impl SectorBuilder {

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add_section<const N: usize>(mut self, section: Section, edges: [Point2; N]) -> Self {
        self.sections.push(section);
        self.edges.push(Box::new(edges));
        self
    }

    #[must_use]
    pub fn sections(&self) -> &[Section] {
        self.sections.as_slice()
    }

    pub fn edges(&self) -> impl Iterator<Item = &[Point2]> {
        self.edges.iter().map(|v| -> &[Point2] { v })
    }

    /// # Errors
    /// Error if the sector contains holes
    pub fn build(self) -> Result<Sector, String> {
        let mut graph = HashMap::<IdentifierEdgeHalf, EdgeHalf>::new();

        // // Build graph // //
        for (idx_section, edge) in self.edges.iter().enumerate() {
            let section = IdentifierSection::from_raw(idx_section as u16);
            for (i, &prev) in edge.iter().enumerate() {
                let next      = edge[(i + 1) % edge.len()];
                let connected = edge[(i + 2) % edge.len()];
                graph.insert(IdentifierEdgeHalf::new(prev, next), EdgeHalf::new(connected, section));
            }
        }

        // // Check Graph for Islands // //
        let mut bounds = Vec::<IdentifierEdgeHalf>::new();
        let edges  = graph.keys().copied().collect::<Vec<_>>().into_boxed_slice();
        for key in graph.keys() {
            let key_rev = key.with_reverse();
            if !graph.contains_key(&key_rev) {
                if bounds.is_empty() {
                    extract_boundry_from_sector(&mut bounds, key_rev, &edges);
                } else if !bounds.contains(&key_rev) {
                    return Err("Multiple sector boundries detected.".to_owned());
                }
            }
        }

        // // Return Graph // //
        Ok(Sector{graph, sections: self.sections})
    }

}