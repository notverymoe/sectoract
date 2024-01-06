// Copyright 2023 Natalie Baker // AGPLv3 //

use core::fmt::Debug;

use bytemuck::{Zeroable, Pod, ByteHash, ByteEq};

use crate::map::Point2;

#[derive(Clone, Copy, Zeroable, Pod, ByteHash, ByteEq)]
#[repr(transparent)]
pub struct IdentifierEdgeHalf([Point2; 2]);

impl Debug for IdentifierEdgeHalf {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("IdentifierEdge")
            .field(&self.prev())
            .field(&self.next())
            .finish()
    }
}

impl IdentifierEdgeHalf {

    #[must_use]
    pub const fn new(prev: Point2, next: Point2) -> Self {
        Self([prev, next])
    }

    #[must_use]
    pub const fn with_reverse(self) -> Self {
        Self::new(self.next(), self.prev())
    }

    #[must_use]
    pub const fn with_next(self, v: Point2) -> Self {
        constmuck::cast::<u64, Self>(constmuck::cast::<Self, u64>(self) << 32 | (constmuck::cast::<Point2, u32>(v) as u64))
    }

    #[must_use]
    pub const fn is_same_edge(self, other: Self) -> bool {
        self.const_eq(other) || self.with_reverse().const_eq(other)
    }

    #[must_use]
    pub const fn connects_to(self, other: Self) -> bool {
        self.next().const_eq(other.prev()) ||
        self.next().const_eq(other.next()) ||
        self.prev().const_eq(other.next()) ||
        self.prev().const_eq(other.prev())
    }
    
    #[must_use]
    pub const fn prev(self) -> Point2 {
        self.0[0]
    }

    #[must_use]
    pub const fn next(self) -> Point2 {
        self.0[1]
    }

    #[must_use]
    pub const fn const_eq(self, other: Self) -> bool {
        constmuck::cast::<Self, u64>(self) == constmuck::cast::<Self, u64>(other)
    }
}
