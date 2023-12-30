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
        len:  isize,
        height: i16,
    },
    Point{
        len:  isize,
        start: u16,
        slices: Vec<i16>,
    },
    Edge{
        len:  isize,
        start: u16,
        slices: Vec<i16>,
    }
}

impl SectionSlope {

    #[must_use]
    pub const fn flat(len: usize, height: i16) -> Self {
        Self::Flat{ len: len as isize, height }
    }

    #[must_use]
    pub const fn slope_from_point(len: usize, start: u16, slices: Vec<i16>) -> Self {
        Self::Point { len: len as isize, start, slices }
    }

    #[must_use]
    pub const fn slope_from_edge(len: usize, start: u16, slices: Vec<i16>) -> Self {
        Self::Edge { len: len as isize, start, slices }
    }

}

impl SectionSlope {

    #[must_use]
    pub fn slice_iter(&self) -> SectionSlopeIter {
        let (wrap_idx, offset_max, start): (isize, isize, [isize; 2]) = match self {
            SectionSlope::Flat  { len,        .. } => { (*len, (len+1)/2, [0, 0]) },
            SectionSlope::Point { len, start, .. } => { (*len, (len+1)/2, [*start as isize, *start as isize]) },
            SectionSlope::Edge  { len, start, .. } => { (*len,     len/2, [*start as isize, *start as isize + 1]) },
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
            SectionSlope::Flat { len, .. } |
            SectionSlope::Point{ len, .. } => ((len+1)/2) as usize,
            SectionSlope::Edge { len, .. } => (len/2) as usize,
        }
    }

    #[must_use]
    pub fn slice_height_for_point(&self, point: usize) -> i16 {
        self.slice_height(self.slice_idx(point))
    }

    #[must_use]
    pub fn slice_height(&self, slice: usize) -> i16 {
        match self {
            SectionSlope::Flat { height, .. } => *height,
            SectionSlope::Point{ slices, .. } |
            SectionSlope::Edge { slices, .. } => slices[slice],
        }
    }

    #[must_use]
    pub fn slice_idx(&self, point: usize) -> usize {
        if let SectionSlope::Flat { .. } = self {
            0
        } else {
            let point = point as isize;
            let iter = self.slice_iter();
    
            let dist_a = if iter.start[0] >= point {
                iter.start[0] - point
            } else {
                (iter.start[0] + iter.wrap_idx) - point
            };
    
            let dist_b = if point >= iter.start[1] {
                point - iter.start[1]
            } else {
                (point + iter.wrap_idx) - iter.start[1]
            };
    
            dist_a.min(dist_b) as usize
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