// Copyright 2023 Natalie Baker // AGPLv3 //

use core::fmt::Debug;

use bytemuck::{Zeroable, Pod, ByteHash, ByteEq};

use crate::map::SectorPoint2;

#[derive(Clone, Copy, Zeroable, Pod, ByteHash, ByteEq)]
#[repr(transparent)]
pub struct IdentifierEdgeHalf([SectorPoint2; 2]);

impl Debug for IdentifierEdgeHalf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f
            .debug_tuple("IdentifierEdge")
            .field(&self.prev())
            .field(&self.next())
            .finish()
    }
}

impl IdentifierEdgeHalf {

    #[must_use]
    pub const fn new(v: [SectorPoint2; 2]) -> Self {
        Self(v)
    }

    #[must_use]
    pub const fn with_reverse(self) -> Self {
        Self::new([self.0[0], self.0[1]])
    }

    #[must_use]
    pub const fn with_next(self, v: SectorPoint2) -> Self {
        Self::new([self.0[0], v])
    }
    
    #[must_use]
    pub const fn prev(self) -> SectorPoint2 {
        self.0[1]
    }

    #[must_use]
    pub const fn next(self) -> SectorPoint2 {
        self.0[0]
    }

}
