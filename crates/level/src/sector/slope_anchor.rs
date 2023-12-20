// Copyright 2023 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Clone, Copy)]
pub struct SlopeAnchor(u8);

impl SlopeAnchor {

    pub fn from_point(idx: usize) -> Self {
        assert!(idx < 128, "Anchor index can only include indicies [0, 127]");
        Self((idx << 1) as u8)
    }

    pub fn from_edge(idx: usize) -> Self {
        assert!(idx < 128, "Anchor index can only include indicies [0, 127]");
        Self(((idx << 1) | 0x01) as u8)
    }

    pub fn is_point(&self) -> bool {
        self.0 % 2 == 0
    }

    pub fn is_edge(&self) -> bool {
        !self.is_point()
    }

    pub fn index_pair(&self) -> [usize; 2] {
        let idx = self.to_raw();
        [
            idx,
            if self.is_point() { idx } else { idx + 1 },
        ]
    }

    pub fn to_slope_iter(&self, len: usize) -> SlopeIterator {
        SlopeIterator{
            start: self.index_pair().map(|v| v as isize),
            offset: 0,
            offset_max: (len/2 + if self.is_point() { 1 } else { 0 }) as isize,
            index_max: len as isize,
        }
    }

    pub fn to_quad_iter(&self, len: usize) -> SlopeQuadIterator {
        self.to_slope_iter(len).to_quad_iter()
    }

    pub fn to_raw(&self) -> usize {
        (self.0 / 2) as usize
    }

}

#[derive(Debug, Default, Clone, Copy)]
pub struct SlopeIterator {
    start: [isize; 2],
    offset:     isize,
    offset_max: isize,
    index_max:  isize,
}

impl SlopeIterator {

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

    pub fn to_quad_iter(self) -> SlopeQuadIterator {
        SlopeQuadIterator{inner: self}
    }

}

impl Iterator for SlopeIterator {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.peek();
        if result.is_some() {
            self.offset += 1;
        }
        result
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SlopeQuadIterator {
    inner: SlopeIterator
}

impl Iterator for SlopeQuadIterator {
    type Item = [[usize; 2]; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(next), Some(peek)) = (self.inner.next(), self.inner.peek()) {
            return Some([next, peek]);
        }
        None
    }
}