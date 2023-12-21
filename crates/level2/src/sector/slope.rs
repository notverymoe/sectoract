// Copyright 2023 Natalie Baker // AGPLv3 //

use tinyvec::SliceVec;

use crate::{to_world_pos, SlopeAnchor, SlopeKind, CapTessIter, CapSliceIter};

#[derive(Debug, Default, Clone, Copy)]
pub struct SectorSlope {
    pub anchor: SlopeAnchor,
    pub start:  i16,
    pub end:   [i16; 2],
}

impl SectorSlope {

    pub fn new(anchor: SlopeAnchor, start: i16, end: [i16; 2]) -> Self {
        Self{anchor, start, end}
    }

    pub fn apply_to(&self, points: &mut [[f32; 3]], kind: SlopeKind) {
        let slope_start  = to_world_pos([0, self.start][kind as usize]);  
        let slope_height = to_world_pos(self.end[kind as usize]);

        let iter      = CapSliceIter::from_anchor(self.anchor, points.len());
        let slope_inc = slope_height/(iter.count() as f32);

        for (offset, [idx_0, idx_1]) in iter.enumerate() {
            points[idx_0][2] = slope_start + slope_inc*(offset as f32);
            points[idx_1][2] = slope_start + slope_inc*(offset as f32);
        }
    }

    pub fn generate_cap_triangles(&self, len: usize, reverse: bool, out: &mut SliceVec<[usize; 3]>) {
        for [tri_0, tri_1] in CapTessIter::from_anchor(self.anchor, len, reverse) {
            if let Some(tri_0) = tri_0 {
                out.push(tri_0);
            }

            if let Some(tri_1) = tri_1 {
                out.push(tri_1);
            }
        }
    }

    pub fn tessslate_cap(&self, points: &[[f32; 3]], reverse: bool, out: &mut SliceVec<[[f32; 3]; 3]>) {
        for [tri_0, tri_1] in CapTessIter::from_anchor(self.anchor, points.len(), reverse) {
            if let Some(tri_0) = tri_0 {
                out.push(tri_0.map(|i| points[i]));
            }

            if let Some(tri_1) = tri_1 {
                out.push(tri_1.map(|i| points[i]));
            }
        }
    }

}

