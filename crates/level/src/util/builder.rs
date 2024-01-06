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

    pub fn add_section<const N: usize>(&mut self, section: Section, edges: [Point2; N]) -> &mut Self {
        self.sections.push(section);
        self.edges.push(Box::new(edges));
        self
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
                graph.insert(IdentifierEdgeHalf::new([prev, next]), EdgeHalf{next, section});
            }
        }

        // // Check Graph for Islands // //
        let mut bounds = Vec::<IdentifierEdgeHalf>::new();
        for key in graph.keys() {
            let key_rev = key.with_reverse();
            if !graph.contains_key(&key_rev) {
                if bounds.is_empty() {
                    bounds.push(key_rev);
                    loop {
                        let test_key = bounds[bounds.len() - 1];
                        if let Some(next_key) = graph.keys().map(|v| v.with_reverse()).find(|&v| 
                                test_key != v &&                         // Exclude the key we're looking for
                                test_key.connects_to(v) &&               // Check that we have connectivity
                                !graph.contains_key(&v) &&               // Check the graph doesn't contain the key (is orphan edge)
                                (bounds[0] == v || !bounds.contains(&v)) // Check that we haven't already added it, unless it's the first key
                            ) {

                            // We have closed the loop!
                            if bounds[0] == next_key { break; }

                            bounds.push(next_key);
                        } else {
                            return Err("Incomplete or duplicate sector boundry detected. This shouldn't be possible.".to_owned());
                        }
                    }
                    // Populate bounds
                } else if !bounds.contains(&key_rev) {
                    return Err("Multiple sector boundries detected.".to_owned());
                }
            }
        }

        // // Return Graph // //
        Ok(Sector{graph, sections: self.sections})
    }

}