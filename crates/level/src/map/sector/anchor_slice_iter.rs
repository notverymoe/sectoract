// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::SectorAnchor;

#[derive(Debug, Default, Clone)]
pub struct AnchorSliceIter {
    start: [isize; 2],
    offset:     isize,
    offset_max: isize,
    index_max:  isize,
}

impl AnchorSliceIter {
    #[must_use]
    pub fn from_anchor(anchor: SectorAnchor, len: usize) -> Self {
        Self{
            start: anchor.index_pair().map(|v| v as isize), 
            offset: 0, 
            offset_max: if anchor.is_point() { len/2 + 1 } else { len/2 } as isize,
            index_max: len as isize
        }
    }
    
    #[must_use]
    pub const fn peek(&self) -> Option<[usize; 2]> {
        if self.offset >= self.offset_max {
            None
        }  else {
            Some([
                (self.start[0] - self.offset).rem_euclid(self.index_max) as usize,
                (self.start[1] + self.offset).rem_euclid(self.index_max) as usize,
            ])
        }
    }

    #[must_use]
    pub const fn max_iter(&self) -> usize {
        self.offset_max as usize
    }
}

impl Iterator for AnchorSliceIter {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() { self.offset += 1; }
        result
    }
}