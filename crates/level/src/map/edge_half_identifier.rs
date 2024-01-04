// Copyright 2023 Natalie Baker // AGPLv3 //

use core::fmt::Debug;

use crate::map::IdentifierSectorPoint;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentifierEdgeHalf(u32);

impl Debug for IdentifierEdgeHalf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f
            .debug_tuple("IdentifierEdge")
            .field(&usize::from(self.prev()))
            .field(&usize::from(self.next()))
            .finish()
    }
}

impl IdentifierEdgeHalf {

    #[must_use]
    pub const fn new(v: [IdentifierSectorPoint; 2]) -> Self {
        Self(((v[0].to_raw() as u32) << 16) | (v[1].to_raw() as u32))
    }

    #[must_use]
    pub const fn with_reverse(self) -> Self {
        Self((self.0 >> 16) | (self.0 << 16))
    }

    #[must_use]
    pub const fn with_next(self, v: IdentifierSectorPoint) -> Self {
        Self((self.0 << 16) | (v.to_raw() as u32))
    }
    
    #[must_use]
    pub const fn prev(self) -> IdentifierSectorPoint {
        IdentifierSectorPoint::from_raw((self.0 >> 16) as u16)
    }

    #[must_use]
    pub const fn next(self) -> IdentifierSectorPoint {
        IdentifierSectorPoint::from_raw(self.0 as u16)
    }

    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }

}
