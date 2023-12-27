// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{SectorAnchor, IdentifierMaterial};

use super::AnchorSliceIter;

#[derive(Debug, Clone)]
pub enum SectorSurface {
    Flat(SectorFlat),
    Sloped(SectorSlope),
}

impl SectorSurface {

    pub fn resolve_heights(&self, out: &mut [i16]) -> bool {
        match self {
            SectorSurface::Flat(v)   => v.resolve_heights(out),
            SectorSurface::Sloped(v) => v.resolve_heights(out),
        }
    }

    #[must_use] 
    pub fn height_range(&self) -> [i16; 2] {
        match self {
            SectorSurface::Flat(v)   => v.height_range(),
            SectorSurface::Sloped(v) => v.height_range(),
        }
    }

    #[must_use] 
    pub fn materials(&self) -> &[IdentifierMaterial] {
        match self {
            SectorSurface::Flat(v)   => core::slice::from_ref(&v.material),
            SectorSurface::Sloped(v) => &v.materials,
        }
    }

}


#[derive(Debug, Clone, Copy)]
pub struct SectorFlat {
    pub height:   i16,
    pub material: IdentifierMaterial,
}

impl SectorFlat {

    #[must_use]
    pub fn resolve_heights(&self, out: &mut [i16]) -> bool {
        out.fill(self.height);
        true
    }

    #[must_use]
    pub const fn height_range(&self) -> [i16; 2] {
        [self.height, self.height]
    }

}

#[derive(Debug, Clone)]
pub struct SectorSlope {
    pub anchor:   SectorAnchor,
    pub range:     [i16; 2],
    pub slices:    Vec<i16>,
    pub materials: Vec<IdentifierMaterial>,
}

impl SectorSlope {

    #[must_use]
    pub fn resolve_heights(&self, out: &mut [i16]) -> bool {
        let iter = AnchorSliceIter::from_anchor(self.anchor, out.len());
        if iter.max_iter() == self.slices.len() {
            for (offset, [idx_0, idx_1]) in iter.enumerate() {
                out[idx_0] = self.slices[offset];
                out[idx_1] = self.slices[offset];
            }
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn height_range(&self) -> [i16; 2] {
        let mut result = [self.slices[0], self.slices[0]];
        for &height in &self.slices[0..] {
            result[0] = result[0].min(height);
            result[1] = result[1].max(height);
        }
        result
    }

}