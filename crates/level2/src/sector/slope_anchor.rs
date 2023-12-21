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

    pub fn to_raw(&self) -> usize {
        (self.0 / 2) as usize
    }

}