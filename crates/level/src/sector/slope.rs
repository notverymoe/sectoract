// Copyright 2023 Natalie Baker // AGPLv3 //

use tinyvec::SliceVec;

use crate::{to_world_pos, SlopeAnchor, SlopeQuadIterator, SlopeKind};

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

        let iter      = self.anchor.to_slope_iter(points.len());
        let slope_inc = slope_height/(iter.len() as f32);

        for (offset, [idx_0, idx_1]) in iter.enumerate() {
            points[idx_0][2] = slope_start + slope_inc*(offset as f32);
            points[idx_1][2] = slope_start + slope_inc*(offset as f32);
        }
    }

    pub fn generate_cap_triangles(&self, len: usize, reverse: bool, out: &mut SliceVec<[usize; 3]>) {
        for [tri_0, tri_1] in SlopeCapIter::new(self.anchor.to_quad_iter(len), reverse) {
            if let Some(tri_0) = tri_0 {
                out.push(tri_0);
            }

            if let Some(tri_1) = tri_1 {
                out.push(tri_1);
            }
        }
    }

    pub fn tessslate_cap(&self, points: &[[f32; 3]], reverse: bool, out: &mut SliceVec<[[f32; 3]; 3]>) {
        for [tri_0, tri_1] in SlopeCapIter::new(self.anchor.to_quad_iter(points.len()), reverse) {
            if let Some(tri_0) = tri_0 {
                out.push(tri_0.map(|i| points[i]));
            }

            if let Some(tri_1) = tri_1 {
                out.push(tri_1.map(|i| points[i]));
            }
        }
    }

}

pub struct SlopeCapIter {
    inner: SlopeQuadIterator,
    reverse: bool,
}

impl SlopeCapIter {
    pub fn new(inner: SlopeQuadIterator, reverse: bool) -> Self {
        Self{inner, reverse}
    }
}

impl Iterator for SlopeCapIter {
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