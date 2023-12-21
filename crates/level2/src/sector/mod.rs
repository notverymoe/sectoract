// Copyright 2023 Natalie Baker // AGPLv3 //

use tinyvec::SliceVec;

use crate::SectorPoint;

mod slope_anchor;
pub use slope_anchor::*;

mod slope;
pub(crate) use slope::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum SlopeKind {
    Roof  = 0,
    Floor = 1,
}

pub struct Sector {
    pub points: Box<[SectorPoint]>,
    pub slope: SectorSlope,
}

impl Sector {

    pub fn new(
        points: Box<[SectorPoint]>,
        height: i16,
    ) -> Self {
        Self::new_slope(
            points, 
            SlopeAnchor::from_edge(0),
            height as i16, 
            [0, 0]
        )
    }

    pub fn new_slope(
        points: Box<[SectorPoint]>, 
        slope_anchor: SlopeAnchor, 
        slope_start: i16,
        slope_end: [i16; 2]
    ) -> Self {
        Self{
            points,
            slope: SectorSlope::new(slope_anchor, slope_start, slope_end)
        }
    }

    pub fn is_flat(&self, kind: SlopeKind) -> bool {
        self.slope.end[kind as usize] == 0
    }

    pub fn is_sloped(&self, kind: SlopeKind) -> bool {
        !self.is_flat(kind)
    }

    pub fn generate_wall_loop(&self, kind: Option<SlopeKind>, buffer: &mut SliceVec<[f32; 3]>) {
        buffer.clear();
        buffer.extend(self.points.iter().map(SectorPoint::to_world));
        if let Some(kind) = kind {
            self.slope.apply_to(buffer.as_mut_slice(), kind);
        }
    }

    pub fn generate_surface(&self, kind: SlopeKind, buffer_loop: &mut SliceVec<[f32; 3]>, buffer_surface: &mut SliceVec<[[f32; 3]; 3]>) {
        self.generate_wall_loop(Some(kind), buffer_loop);
        self.slope.tessslate_cap(buffer_loop, kind == SlopeKind::Roof, buffer_surface);
    }
    
}