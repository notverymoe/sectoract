// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{SlopeAnchor, CapSliceIter};

#[derive(Debug, Default, Clone, Copy)]
pub struct CapTessIter {
    inner: CapSliceIter,
    flip_order: bool,

}

impl CapTessIter {
    pub fn from_anchor(anchor: SlopeAnchor, len: usize, flip_order: bool) -> Self {
        Self{
            inner: CapSliceIter::from_anchor(anchor, len), 
            flip_order
        }
    }
}

impl Iterator for CapTessIter {
    type Item = [Option<[usize; 3]>; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some([idx_00, idx_01]), Some([idx_10, idx_11])) = (self.inner.next(), self.inner.peek()) {
            if self.flip_order { 
                Some([
                    (idx_00 != idx_01).then_some([idx_11, idx_01, idx_00]),
                    (idx_10 != idx_11).then_some([idx_10, idx_11, idx_00])
                ])
            } else { 
                Some([
                    (idx_00 != idx_01).then_some([idx_00, idx_01, idx_11]),
                    (idx_10 != idx_11).then_some([idx_00, idx_11, idx_10])
                ])
            }
        } else {
            None
        }
    }
}
