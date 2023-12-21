// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::SlopeAnchor;

#[derive(Debug, Default, Clone, Copy)]
pub struct CapSliceIter {
    start: [isize; 2],
    offset:     isize,
    offset_max: isize,
    index_max:  isize,
}

impl CapSliceIter {
    pub fn from_anchor(anchor: SlopeAnchor, len: usize) -> Self {
        Self{
            start: anchor.index_pair().map(|v| v as isize), 
            offset: 0, 
            offset_max: if anchor.is_point() { len/2 + 1 } else { len/2 } as isize,
            index_max: len as isize
        }
    }
    
    pub fn peek(&self) -> Option<[usize; 2]> {
        if self.offset >= self.offset_max {
            None
        }  else {
            Some([
                (self.start[0] - self.offset).rem_euclid(self.index_max) as usize,
                (self.start[1] + self.offset).rem_euclid(self.index_max) as usize,
            ])
        }
    }
}

impl Iterator for CapSliceIter {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() { self.offset += 1; }
        result
    }
}