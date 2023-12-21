// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{Sector, SlopeKind, SectorPoint};

mod cap_slice_iter;
pub use cap_slice_iter::*;

mod cap_tess_iter;
pub use cap_tess_iter::*;
use tinyvec::SliceVec;

pub fn tesselate_sector_cap(
    sector: &Sector, 
    kind: Option<SlopeKind>, 
    buffer_points: &mut SliceVec<[f32; 3]>, 
    buffer_index: &mut SliceVec<[usize; 3]>
) {
    // Add points
    let points_start = buffer_points.len();
    buffer_points.extend(sector.points.iter().map(SectorPoint::to_world));
    if let Some(kind) = kind {
        sector.slope.apply_to(&mut buffer_points[points_start..], kind);
    }

    // Add indicies
    let points_len = buffer_points.len() - points_start;
    for tris in CapTessIter::from_anchor(sector.slope.anchor, points_len, kind != Some(SlopeKind::Roof)) {
        buffer_index.extend(tris.into_iter().flatten().map(|v| v.map(|v| points_start + v)));
    }
}