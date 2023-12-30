// Copyright 2023 Natalie Baker // AGPLv3 //

use super::{IdentifierSection, IdentifierPoint};

#[derive(Debug, Clone)]
pub struct Section {
    pub parent:   Option<IdentifierSection>,
    pub surfaces: Option<[SectionSlope; 2]>,
    pub edges:    Vec<IdentifierPoint>,
}

#[derive(Debug, Clone)]
pub enum SectionSlope {
    Flat{
        len:  usize,
        height: i16,
    },
    Point{
        start: u16,
        slices: Vec<i16>,
    },
    Edge{
        start: u16,
        slices: Vec<i16>,
    }
}

impl SectionSlope {

    #[must_use]
    pub const fn flat(len: usize, height: i16) -> Self {
        Self::Flat{len, height}
    }

    #[must_use]
    pub const fn slope_from_point(start: u16, slices: Vec<i16>) -> Self {
        Self::Point { start, slices }
    }

    #[must_use]
    pub const fn slope_from_edge(start: u16, slices: Vec<i16>) -> Self {
        Self::Edge { start, slices }
    }

}

impl SectionSlope {

    #[must_use]
    pub fn slice_iter(&self) -> SectionSlopeIter {
        let (wrap_idx, offset_max, start): (isize, isize, [isize; 2]) = match self {
            SectionSlope::Flat  { len,           .. } => { let len =         *len as isize; (len, (len+1)/2, [0, 0]) },
            SectionSlope::Point { start, slices, .. } => { let len = slices.len() as isize; (len, (len+1)/2, [*start as isize, *start as isize]) },
            SectionSlope::Edge  { start, slices, .. } => { let len = slices.len() as isize; (len,     len/2, [*start as isize, *start as isize + 1]) },
        };
        SectionSlopeIter{
            start,
            offset: 0,
            offset_max,
            wrap_idx,
            slope: self,
        }
    }

    #[must_use]
    pub fn calc_range(&self) -> [i16; 2] {
        match self {
            SectionSlope::Flat { height, .. } => [*height, *height],
            SectionSlope::Point{ slices, .. } |
            SectionSlope::Edge { slices, .. } => slices.iter().fold([i16::MAX, i16::MIN], |[p0, p1], &v| [p0.min(v), p1.max(v)]),
        }
    }

    #[must_use]
    pub fn slice_len(&self) -> usize {
        match self {
            SectionSlope::Flat { len,    .. } => *len,
            SectionSlope::Point{ slices, .. } |
            SectionSlope::Edge { slices, .. } => slices.len(),
        }
    }

    #[must_use]
    pub fn slice_height(&self, slice: usize) -> i16 {
        match self {
            SectionSlope::Flat { height, .. } => *height,
            SectionSlope::Point{ slices, .. } |
            SectionSlope::Edge { slices, .. } => slices[slice],
        }
    }

}


pub struct SectionSlopeIter<'a> {
    start:     [isize; 2],
    offset:     isize,
    offset_max: isize,
    wrap_idx:   isize,
    slope:    &'a SectionSlope,
}

impl<'a> SectionSlopeIter<'a> {

    #[must_use]
    pub fn peek(&self) -> Option<(i16, [usize; 2])> {
        if self.offset >= self.offset_max {
            None
        }  else {
            Some((
                self.slope.slice_height(self.offset as usize), 
                [
                    (self.start[0] - self.offset).rem_euclid(self.wrap_idx) as usize,
                    (self.start[1] + self.offset).rem_euclid(self.wrap_idx) as usize,
                ]
            ))
        }
    }

}

impl<'a> Iterator for SectionSlopeIter<'a> {
    type Item = (i16, [usize; 2]);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() { self.offset += 1; }
        result
    }
}