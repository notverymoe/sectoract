// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::map::{IdentifierEdgeHalf, EdgeHalf};

#[must_use]
pub struct SectionIter<'a, S: core::hash::BuildHasher> {
    graph:  &'a HashMap<IdentifierEdgeHalf, EdgeHalf, S>,
    start:   IdentifierEdgeHalf,
    current: IdentifierEdgeHalf,
    done:    bool,
}

impl<'a, S: core::hash::BuildHasher> SectionIter<'a, S> {

    pub const fn new(
        graph: &'a HashMap<IdentifierEdgeHalf, EdgeHalf, S>,
        start: IdentifierEdgeHalf,
    ) -> Self {
        Self{graph, start, current: start, done: false}
    }

}

impl<'a, S: core::hash::BuildHasher> Iterator for SectionIter<'a, S> {
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