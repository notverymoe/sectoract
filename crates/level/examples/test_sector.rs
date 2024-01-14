// Copyright 2023 Natalie Baker // AGPLv3 //

use sectoract_level::{
    util::{SectorBuilder, extract_section_points_from_sector}, 
    map::{Section, Point2, UNIT_WORLD_I, Surface, IdentifierSection, IdentifierEdgeHalf, Point3}
};

use crate::util::{polys_to_svg, append_ngon_to_obj_str};

mod util;

pub const HEIGHT_MALL:     i16 = UNIT_WORLD_I * 22;
pub const LENGTH_MALL:     i16 = UNIT_WORLD_I * 36;
pub const WIDTH_PLATFORM:  i16 = UNIT_WORLD_I * 3;
pub const WIDTH_CROSSOVER: i16 = UNIT_WORLD_I * 6;

pub fn main() {

    let sector = SectorBuilder::new()
        .add_section(
            Section::new(vec![
                Surface::flat(                0),

                Surface::flat(UNIT_WORLD_I *  3),
                Surface::flat(UNIT_WORLD_I *  4),

                Surface::flat(UNIT_WORLD_I *  9),
                Surface::flat(UNIT_WORLD_I * 10),

                Surface::flat(UNIT_WORLD_I * 15),
                Surface::flat(UNIT_WORLD_I * 16),

                Surface::flat(HEIGHT_MALL),
            ]), 
            Point2::from_slice_const([
                [          0,              0],
                [LENGTH_MALL,              0],
                [LENGTH_MALL, WIDTH_PLATFORM],
                [LENGTH_MALL - WIDTH_CROSSOVER, WIDTH_PLATFORM],
                [              WIDTH_CROSSOVER, WIDTH_PLATFORM],
                [          0, WIDTH_PLATFORM],
            ])
        )
        .add_section(
            Section::flat_with_roof(0, HEIGHT_MALL), 
            Point2::from_slice_const([
                [              WIDTH_CROSSOVER,                   WIDTH_PLATFORM],
                [LENGTH_MALL - WIDTH_CROSSOVER,                   WIDTH_PLATFORM],
                [LENGTH_MALL - WIDTH_CROSSOVER, WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [              WIDTH_CROSSOVER, WIDTH_CROSSOVER + WIDTH_PLATFORM],
            ])
        )
        .add_section(
            Section::new(vec![
                Surface::flat(                0),

                Surface::flat(UNIT_WORLD_I *  6),
                Surface::flat(UNIT_WORLD_I *  7),

                Surface::flat(UNIT_WORLD_I * 12),
                Surface::flat(UNIT_WORLD_I * 13),

                Surface::flat(UNIT_WORLD_I * 18),
                Surface::flat(UNIT_WORLD_I * 19),

                Surface::flat(HEIGHT_MALL),
            ]), 
            Point2::from_slice_const([
                [          0, WIDTH_CROSSOVER +   WIDTH_PLATFORM],
                [              WIDTH_CROSSOVER, WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [LENGTH_MALL - WIDTH_CROSSOVER, WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [LENGTH_MALL, WIDTH_CROSSOVER +   WIDTH_PLATFORM],
                [LENGTH_MALL, WIDTH_CROSSOVER + 2*WIDTH_PLATFORM],
                [          0, WIDTH_CROSSOVER + 2*WIDTH_PLATFORM],
            ])
        )
        .add_section(
            Section::new(vec![
                // Surface::flat(                0),

                // Surface::slope(
                //     Point2::new(0, WIDTH_PLATFORM),
                //     Point2::new(0, WIDTH_CROSSOVER),
                //     [UNIT_WORLD_I *  3, UNIT_WORLD_I *  6]
                // ),
                Surface::slope(
                    Point2::new(0, WIDTH_PLATFORM),
                    Point2::new(0, WIDTH_CROSSOVER),
                    [UNIT_WORLD_I *  4, UNIT_WORLD_I *  7]
                ),

                Surface::slope(
                    Point2::new(0, WIDTH_PLATFORM),
                    Point2::new(0, WIDTH_CROSSOVER),
                    [UNIT_WORLD_I *  9, UNIT_WORLD_I * 12]
                ),
                Surface::slope(
                    Point2::new(0, WIDTH_PLATFORM),
                    Point2::new(0, WIDTH_CROSSOVER),
                    [UNIT_WORLD_I * 10, UNIT_WORLD_I * 13]
                ),

                Surface::slope(
                    Point2::new(0, WIDTH_PLATFORM),
                    Point2::new(0, WIDTH_CROSSOVER),
                    [UNIT_WORLD_I * 15, UNIT_WORLD_I * 18]
                ),
                Surface::slope(
                    Point2::new(0, WIDTH_PLATFORM),
                    Point2::new(0, WIDTH_CROSSOVER),
                    [UNIT_WORLD_I * 16, UNIT_WORLD_I * 19]
                ),

                Surface::flat(HEIGHT_MALL),
            ]), 
            Point2::from_slice_const([
                [              0, WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [              0, WIDTH_PLATFORM                  ],
                [WIDTH_CROSSOVER, WIDTH_PLATFORM                  ],
                [WIDTH_CROSSOVER, WIDTH_CROSSOVER + WIDTH_PLATFORM],
            ])
        )
        .add_section(
            Section::new(vec![
                Surface::slope(
                    Point2::new(0, WIDTH_CROSSOVER + WIDTH_PLATFORM),
                    Point2::new(0, -WIDTH_CROSSOVER),
                    [           0, UNIT_WORLD_I *  4]
                ),

                Surface::slope(
                    Point2::new(0, WIDTH_CROSSOVER + WIDTH_PLATFORM),
                    Point2::new(0, -WIDTH_CROSSOVER),
                    [UNIT_WORLD_I *  6, UNIT_WORLD_I * 9]
                ),
                Surface::slope(
                    Point2::new(0, WIDTH_CROSSOVER + WIDTH_PLATFORM),
                    Point2::new(0, -WIDTH_CROSSOVER),
                    [UNIT_WORLD_I *  7, UNIT_WORLD_I *  10]
                ),

                Surface::slope(
                    Point2::new(0, WIDTH_CROSSOVER + WIDTH_PLATFORM),
                    Point2::new(0, -WIDTH_CROSSOVER),
                    [UNIT_WORLD_I * 12, UNIT_WORLD_I * 15]
                ),
                Surface::slope(
                    Point2::new(0, WIDTH_CROSSOVER + WIDTH_PLATFORM),
                    Point2::new(0, -WIDTH_CROSSOVER),
                    [UNIT_WORLD_I * 13, UNIT_WORLD_I *  16]
                ),
                

                Surface::flat(HEIGHT_MALL),
            ]), 
            Point2::from_slice_const([
                [LENGTH_MALL,                   WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [LENGTH_MALL - WIDTH_CROSSOVER, WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [LENGTH_MALL - WIDTH_CROSSOVER, WIDTH_PLATFORM                  ],
                [LENGTH_MALL,                   WIDTH_PLATFORM                  ],
            ])
        );

    polys_to_svg(sector.edges(), "test_export_dir/out_builder.svg");

    let sector = sector.build().unwrap();
    // println!("{:#?}", sector);

    let edges: Vec<_> = sector.graph.keys().map(|v| vec![v.prev(), v.next()].into_boxed_slice()).collect();
    polys_to_svg(edges.iter().map(|v| -> &[Point2] { v }), "test_export_dir/out_graph.svg");

    // // Extract Sector Contours // //
    let mut section_pnts: Vec<Vec<Point2>> = Vec::with_capacity(sector.sections.len());
    for section in 0..sector.sections.len() {
        section_pnts.push(extract_section_points_from_sector(
            &sector.graph, 
            IdentifierSection::from_raw(section as u16)
        ).unwrap());
    }

    polys_to_svg(section_pnts.iter().map(|v| -> &[Point2] { v }), "test_export_dir/out_rebuilt.svg");

    // // Build Surfaces // //

    let mut obj_str = "".to_owned();
    let mut vert_count = 0;
    for (section, points) in sector.sections.iter().zip(section_pnts.iter()) {
        for (i, surface) in section.surfaces.iter().enumerate() {

            vert_count = if i % 2 == 0 {
                append_ngon_to_obj_str(&mut obj_str, vert_count, points.iter().map(|&v| v.extend(surface.get_height_at_point(v))))
            } else {
                append_ngon_to_obj_str(&mut obj_str, vert_count, points.iter().rev().map(|&v| v.extend(surface.get_height_at_point(v))))
            };

            build_section_edges(
                i, section, points, 
                |e| sector.graph.get(&e).map(|v| &sector.sections[usize::from(v.section)]),
                |_e, f| { vert_count = append_ngon_to_obj_str(&mut obj_str, vert_count, f.iter().copied()) }
            );
        }
    }

    std::fs::write("test_export_dir/out_surfaces.obj", obj_str).unwrap();
}

fn build_section_edges<'a>(
    part:    usize,
    section: &Section,
    points:  &[Point2],
    try_get_connection: impl Fn(IdentifierEdgeHalf) -> Option<&'a Section>,
    mut push_edge:      impl FnMut(IdentifierEdgeHalf, &[Point3]),
) {
    let is_floor = part % 2 == 0;
    if !is_floor {
        return; // TODO ceiling to neighbour
    }

    let surf_curr = &section.surfaces[part];

    // TODO triangle edges
    for edge in points.iter().enumerate().map(|(i, &v)| IdentifierEdgeHalf::new(v, points[(i+1) % points.len()])) {
        let height_edge = surf_curr.get_height_at_edge(edge);
        if let Some(height_targ) = get_target_heights(part, section, edge, height_edge, &try_get_connection) {
            match (height_edge[0] == height_targ[0], height_edge[1] == height_targ[1]) {
                (true, true) => continue, // Edge matches
                (true, false) => {
                    // Tri
                    push_edge(edge, &[
                        edge.prev().extend(height_edge[0]),
                        edge.next().extend(height_targ[1]),
                        edge.next().extend(height_edge[1]),
                    ]);
                },
                (false, true) => {
                    // Tri
                    push_edge(edge, &[
                        edge.prev().extend(height_edge[0]),
                        edge.prev().extend(height_targ[0]),
                        edge.next().extend(height_edge[1]),
                    ]);
                },
                (false, false) => {
                    // Quad
                    push_edge(edge, &[
                        edge.prev().extend(height_edge[0]),
                        edge.prev().extend(height_targ[0]),
                        edge.next().extend(height_targ[1]),
                        edge.next().extend(height_edge[1]),
                    ]);
                }
            }

        }


    }
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
            if !is_edge_occluded_by_section(section_other, edge, height_floor, height_ceil) {
                Some(height_ceil)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        // SAFE: This function cannot be called on ceilings
        Some(section.surfaces[part+1].get_height_at_edge(edge))
    }
}

fn is_edge_under(a: [i16; 2], b: [i16; 2]) -> bool {
    a[0] <= b[0] && a[1] <= b[1]
}


fn is_edge_occluded_by_surfaces(
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
