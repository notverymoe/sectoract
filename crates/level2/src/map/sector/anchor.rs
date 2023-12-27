// Copyright 2023 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SectorAnchor(u16);

impl SectorAnchor {

    pub const MAX_IDX: usize = (u16::MAX >> 1) as usize;

    /// # Panics
    /// If idx is > 32767
    #[must_use]
    pub const fn from_point(idx: usize) -> Self {
        assert!(idx <= Self::MAX_IDX, "Anchor index can only include indicies (u16::MAX >> 1)");
        Self((idx << 1) as u16)
    }

    /// # Panics
    /// If idx is > 32767
    #[must_use]
    pub const fn from_edge(idx: usize) -> Self {
        assert!(idx <= Self::MAX_IDX, "Anchor index can only include indicies (u16::MAX >> 1)");
        Self(((idx << 1) | 0x0001) as u16)
    }

    #[must_use]
    pub const fn is_point(&self) -> bool {
        self.0 % 2 == 0
    }

    #[must_use]
    pub const fn is_edge(&self) -> bool {
        !self.is_point()
    }

    #[must_use]
    pub const fn index_pair(&self) -> [usize; 2] {
        let idx = self.to_raw();
        [
            idx,
            if self.is_point() { idx } else { idx + 1 },
        ]
    }

    #[must_use]
    pub const fn to_raw(&self) -> usize {
        (self.0 / 2) as usize
    }

}