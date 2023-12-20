// Copyright 2023 Natalie Baker // AGPLv3 //

pub struct SectorCapIter {
    inner: SectorQuadIterator,
    reverse: bool,
}

impl SectorCapIter {
    pub fn new(inner: SectorQuadIterator, reverse: bool) -> Self {
        Self{inner, reverse}
    }
}

impl Iterator for SectorCapIter {
    type Item = [Option<[usize; 3]>; 2];
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(
            |[[idx_00, idx_01], [idx_10, idx_11]]| if self.reverse { 
                [
                    (idx_00 != idx_01).then_some([idx_11, idx_01, idx_00]),
                    (idx_10 != idx_11).then_some([idx_10, idx_11, idx_00])
                ]
            } else { 
                [
                    (idx_00 != idx_01).then_some([idx_00, idx_01, idx_11]),
                    (idx_10 != idx_11).then_some([idx_00, idx_11, idx_10])
                ]
            }
        )
    }
}


#[derive(Debug, Default, Clone, Copy)]
pub struct SectorQuadIterator {
    inner: SectorEdgeIterator
}

impl SectorQuadIterator {

    pub fn new(inner: SectorEdgeIterator) -> Self {
        Self{inner}
    }

}

impl Iterator for SectorQuadIterator {
    type Item = [[usize; 2]; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(next), Some(peek)) = (self.inner.next(), self.inner.peek()) {
            return Some([next, peek]);
        }
        None
    }
}


#[derive(Debug, Default, Clone, Copy)]
pub struct SectorEdgeIterator {
    start: [isize; 2],
    offset:     isize,
    offset_max: isize,
    index_max:  isize,
}

impl SectorEdgeIterator {

    pub fn new(start: [isize; 2], offset_max: isize, index_max: isize) -> Self {
        Self { start, offset: 0, offset_max, index_max }
    }

    pub fn len(&self) -> usize {
        self.offset_max as usize
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

impl Iterator for SectorEdgeIterator {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() {
            self.offset += 1;
        }
        result
    }
}