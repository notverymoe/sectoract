// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{IdentifierEdgeHalf, EdgeHalf};

#[must_use]
pub struct SectionIter<'a> {
    graph:  &'a HashMap<IdentifierEdgeHalf, EdgeHalf>,
    start:   IdentifierEdgeHalf,
    current: IdentifierEdgeHalf,
    done:    bool,
}

impl<'a> SectionIter<'a> {

    pub const fn new(
        graph: &'a HashMap<IdentifierEdgeHalf, EdgeHalf>,
        start: IdentifierEdgeHalf,
    ) -> Self {
        Self{graph, start, current: start, done: false}
    }

}

impl<'a> Iterator for SectionIter<'a> {
    type Item = (IdentifierEdgeHalf, EdgeHalf);

    #[allow(clippy::unwrap_in_result)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None; }
        let result   = (self.current, *self.graph.get(&self.current).unwrap());
        self.current = self.current.with_next(result.1.next);
        self.done    = self.start == self.current;
        Some(result)
    }
}