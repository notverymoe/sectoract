// Copyright 2023 Natalie Baker // AGPLv3 //

use std::collections::HashMap;

use crate::{map::{Section, Point2, IdentifierEdgeHalf, EdgeHalf, IdentifierSection}, util::SectionIter};

use super::FaceWriter;

#[must_use]
pub fn extract_section_contour<S: core::hash::BuildHasher>(graph: &HashMap<IdentifierEdgeHalf, EdgeHalf, S>, section: IdentifierSection) -> Option<Vec<Point2>> {
    for (k, v) in graph {
        if v.section != section { continue; }
        return Some(SectionIter::new(graph, *k).map(|(e, _h)| e.prev()).collect());
    }
    None
}

pub fn build_section_faces<'a>(
    section: &Section,
    points: &[Point2],
    try_get_connection: impl Fn(IdentifierEdgeHalf) -> Option<&'a Section>,
    writer: &mut impl FaceWriter,
) {
    for part in 0..section.surfaces.len() {
        build_section_surfaces(part, section, points, writer);
        build_section_edges(part, section, points, &try_get_connection, writer);
    }
}

pub fn build_section_surfaces(
    part:    usize,
    section: &Section,
    points:  &[Point2],
    writer: &mut impl FaceWriter,
) {
    let is_floor = part % 2 == 0;
    let surface = section.surfaces[part];
    if is_floor {
        writer.add_surf(part, &mut points.iter().map(|&v| v.extend(surface.get_height_at_point(v))));
    } else {
        writer.add_surf(part, &mut points.iter().rev().map(|&v| v.extend(surface.get_height_at_point(v))));
    };
}

pub fn build_section_edges<'a>(
    part:    usize,
    section: &Section,
    points:  &[Point2],
    try_get_connection: &impl Fn(IdentifierEdgeHalf) -> Option<&'a Section>,
    writer: &mut impl FaceWriter,
) {
    let surf_curr = &section.surfaces[part];

    let is_top = part + 1 == section.surfaces.len();
    if is_top {
        for edge in points.iter().enumerate().map(|(i, &v)| IdentifierEdgeHalf::new(v, points[(i+1) % points.len()])) {
            let height_edge = surf_curr.get_height_at_edge(edge);
            if let Some(section_other) = try_get_connection(edge.with_reverse()) {
                let surf_neigh = &section_other.surfaces[section_other.surfaces.len() - 1];
                let height_neigh = surf_neigh.get_height_at_edge(edge);
                if is_edge_under(height_neigh, height_edge) {
                    do_push_face(writer, part, edge, height_neigh, height_edge);
                }
            }
        }
    }

    let is_floor = part % 2 == 0;
    if !is_floor { return; }

    for edge in points.iter().enumerate().map(|(i, &v)| IdentifierEdgeHalf::new(v, points[(i+1) % points.len()])) {
        let height_edge = surf_curr.get_height_at_edge(edge);
        if let Some(height_target) = get_target_heights(part, section, edge, height_edge, &try_get_connection) {
            do_push_face(writer, part, edge, height_edge, height_target);
        }
    }
}

fn do_push_face(writer: &mut impl FaceWriter, part: usize, edge: IdentifierEdgeHalf, source: [i16; 2], target: [i16; 2]) {
    let p = [
        edge.prev().extend(source[0]),
        edge.prev().extend(target[0]),
        edge.next().extend(target[1]),
        edge.next().extend(source[1]),
    ];
    match (source[0] == target[0], source[1] == target[1]) {
        (true,  true ) => { /* No face */ }, 
        (true,  false) => { /* Tri */  writer.add_wall(part, edge, [p[0], p[2], p[3]]); },
        (false, true ) => { /* Tri */  writer.add_wall(part, edge, [p[0], p[1], p[3]]); },
        (false, false) => { /* Quad */ writer.add_wall(part, edge, p); }
    };
}

fn get_target_heights<'a>(
    part: usize,
    section: &Section,
    edge: IdentifierEdgeHalf,
    height_floor: [i16; 2],
    try_get_connection:&impl Fn(IdentifierEdgeHalf) -> Option<&'a Section>,
) -> Option<[i16; 2]> {
    if let Some(section_other) = try_get_connection(edge.with_reverse()) {

        let surf_neigh = &section_other.surfaces[0];
        let height_neigh = surf_neigh.get_height_at_edge(edge);

        // If we are below the neighbour's floor, then the edge goes upwards
        if is_edge_under(height_floor, height_neigh) {
            // If the ceiling above is under the neighbour's base then we go up to it,
            // otherwise we only go up to the neighbour's base
            let height_above = section.surfaces[part+1].get_height_at_edge(edge);
            if is_edge_under(height_above, height_neigh) {
                Some(height_above)
            } else {
                Some(height_neigh)
            }
        } else if part != 0 {
            let height_ceil = section.surfaces[part-1].get_height_at_edge(edge);
            (!is_edge_occluded_by_section(section_other, edge, height_floor, height_ceil)).then_some(height_ceil)
        } else {
            None
        }
    } else {
        // SAFE: This function cannot be called on ceilings
        Some(section.surfaces[part+1].get_height_at_edge(edge))
    }
}

const fn is_edge_under(a: [i16; 2], b: [i16; 2]) -> bool {
    a[0] <= b[0] && a[1] <= b[1]
}

const fn is_edge_occluded_by_surfaces(
    height_floor: [i16; 2],
    height_ceil: [i16; 2],
    height_neigh_floor: [i16; 2],
    height_neigh_ceil: Option<[i16; 2]>,
) -> bool {
    if is_edge_under(height_floor, height_neigh_floor) {
        if let Some(height_neigh_ceil) = height_neigh_ceil {
            is_edge_under(height_neigh_ceil, height_ceil)
        } else {
            true // Neighbour base, ceil will be higher guaranteed
        }
    } else {
        false
    }
}

fn is_edge_occluded_by_section(
    section_other: &Section,
    edge: IdentifierEdgeHalf,
    height_floor: [i16; 2],
    height_ceil: [i16; 2],
 ) -> bool {
    section_other.iter_floors().any(|(i, v)| is_edge_occluded_by_surfaces(
        height_floor,
        height_ceil,
        v.get_height_at_edge(edge),
        (i != 0).then(|| section_other.surfaces[i-1].get_height_at_edge(edge))
    ))
}
