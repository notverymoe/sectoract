// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{Point2, Section, Sector, IdentifierEdgeHalf, EdgeHalf, IdentifierSection};

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
                let next = edge[(i + 1) % edge.len()];
                graph.insert(IdentifierEdgeHalf::new(prev, next), EdgeHalf{next, section});
            }
        }

        // // Check Graph for Islands // //
        let mut bounds = Vec::<IdentifierEdgeHalf>::new();
        let edges  = graph.keys().copied().collect::<Vec<_>>().into_boxed_slice();
        for key in graph.keys() {
            let key_rev = key.with_reverse();
            if !graph.contains_key(&key_rev) {
                if bounds.is_empty() {
                    extract_boundry(&mut bounds, key_rev, &edges);
                } else if !bounds.contains(&key_rev) {
                    return Err("Multiple sector boundries detected.".to_owned());
                }
            }
        }

        // // Return Graph // //
        Ok(Sector{graph, sections: self.sections})
    }

}

fn extract_boundry(bounds: &mut Vec<IdentifierEdgeHalf>, start: IdentifierEdgeHalf, keys: &[IdentifierEdgeHalf]) {
    bounds.push(start);
    loop {
        let test_key = bounds[bounds.len() - 1];
        if let Some(next_key) = keys.iter().map(|v| v.with_reverse()).find(|&v| 
                start    != v &&                         // Exclude the start key
                test_key != v &&                         // Exclude the key we're looking for
                test_key.connects_to(v) &&               // Check that we have connectivity
                !keys.contains(&v) &&                    // Check the graph doesn't contain the key (is orphan edge)
                (start == v || !bounds.contains(&v)) // Check that we haven't already added it, unless it's the first key
            ) {

            bounds.push(next_key);

            // We found the end!
            // - A polygon must have at least 3 points
            // - Only two edges should connect to the first point
            if bounds.len() >= 3 && next_key.connects_to(start) {
                break;
            }

        } else {
            panic!("Incomplete or duplicate sector boundry detected. This shouldn't be possible.");
        }
    }
}