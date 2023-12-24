// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{SectorPoint3, SectorAnchor};

pub struct SectorSlope {
    pub anchor: SectorAnchor,
    pub start: i16,
    pub end:   i16,
}

impl SectorSlope {

    pub fn apply_heights(&self, points: &mut [SectorPoint3]) {
        let delta = self.end - self.start;
        let delta = (delta as f32)/(points.len() as f32);

        let [anchor_start, anchor_end] = self.anchor.index_pair().map(|v| v as isize);
        let offset_max = self.anchor.iter_len(points.len()) as isize;

        for offset in 0..offset_max {
            let height = self.start + ((delta*(offset as f32)) as i16);
            let idx_0 = (anchor_start - offset).rem_euclid(points.len() as isize) as usize;
            let idx_1 = (anchor_end   + offset).rem_euclid(points.len() as isize) as usize;
            points[idx_0].z = height;
            points[idx_1].z = height;
        }
    }

}