// Copyright 2023 Natalie Baker // AGPLv3 //

use core::fmt::Debug;

use crate::map::IdentifierPoint;

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
    pub const fn new(v: [IdentifierPoint; 2]) -> Self {
        Self(((v[0].to_raw() as u32) << 16) | (v[1].to_raw() as u32))
    }

    #[must_use]
    pub const fn with_reverse(self) -> Self {
        Self((self.0 >> 16) | (self.0 << 16))
    }

    #[must_use]
    pub const fn with_next(self, v: IdentifierPoint) -> Self {
        Self((self.0 << 16) | (v.to_raw() as u32))
    }
    
    #[must_use]
    pub const fn prev(self) -> IdentifierPoint {
        IdentifierPoint::from_raw((self.0 >> 16) as u16)
    }

    #[must_use]
    pub const fn next(self) -> IdentifierPoint {
        IdentifierPoint::from_raw(self.0 as u16)
    }

}
