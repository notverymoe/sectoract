// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::SectorPoint2;

use std::collections::HashSet;

pub fn combine_sectors(polygons: &[&[SectorPoint2]]) {
    //let mut edges: Vec<[SectorPoint2; 2]> = HashSet::from_iter(polygons.iter().flat_map(|polygon| polygon.iter().enumerate().map(|(i, &p)| [p, polygon[(i+1)%polygon.len()]]))).into_iter().collect();

    let mut fixed: Vec<Vec<SectorPoint2>> = Vec::new();
    for (i, &polygon) in polygons.iter().enumerate() {
        let mut combined = Vec::new();
        for (j, &from) in polygon.iter().enumerate() {
            let to = polygon[(j+1) % polygon.len()];
            let mut candidates: Vec<_> = polygons
                .iter()
                .enumerate()
                .filter(|&(k, _)| k != i)
                .flat_map(|(_, &v)| v.iter().filter_map(move |&point| is_point_on_line(from, to, point).map(|r| (point, r)))).collect();
            candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            candidates.dedup_by(|a, b| a.0 == b.0);

            combined.push(from);
            combined.extend(candidates.iter().map(|v| v.0));
        }
        fixed.push(combined);
    }

    let edges: Vec<[SectorPoint2; 2]> = fixed.iter().flat_map(|polygon| polygon.iter().enumerate().map(|(i, &p)| [p, polygon[(i+1)%polygon.len()]])).collect();

    
}

#[must_use]
const fn is_point_on_line(from: SectorPoint2, to: SectorPoint2, point: SectorPoint2) -> Option<i32> {
    let size_x = ((to.x - from.x) as i32) << 16;
    let size_y = ((to.y - from.y) as i32) << 16;

    let delta_x = (point.x - from.x) as i32;
    let delta_y = (point.y - from.y) as i32;

    let norm_x = size_x/delta_x;
    let norm_y = size_y/delta_y;

    if norm_x > (u16::MAX as i32) && norm_y > (u16::MAX as i32) && norm_x == norm_y {
        Some(norm_x)
    } else {
        None
    }
}