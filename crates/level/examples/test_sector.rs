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
                Surface::flat(                0),

                Surface::slope(
                    Point2::new(0, WIDTH_PLATFORM),
                    Point2::new(0, WIDTH_CROSSOVER),
                    [UNIT_WORLD_I *  3, UNIT_WORLD_I *  6]
                ),
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
                Surface::flat(                0),

                Surface::slope(
                    Point2::new(0, WIDTH_CROSSOVER + WIDTH_PLATFORM),
                    Point2::new(0, -WIDTH_CROSSOVER),
                    [-UNIT_WORLD_I, UNIT_WORLD_I *  3]
                ),
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
                append_ngon_to_obj_str(&mut obj_str, vert_count, points.iter().map(|&v| v.extend(surface.get_height_at(v))))
            } else {
                append_ngon_to_obj_str(&mut obj_str, vert_count, points.iter().rev().map(|&v| v.extend(surface.get_height_at(v))))
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
    let is_end   = part == 0 || part+1 == section.surfaces.len();
    if !is_floor && !is_end {
        return;  // We don't generate surfaces for mid-ceilings
    }

    let surf_curr = &section.surfaces[part];

    // TODO triangle edges

    if !is_end {
        let surf_prev = &section.surfaces[part-1];

        for (i, &pnt_prev) in points.iter().enumerate() {
            let pnt_next = points[(i+1) % points.len()];
            
            let edge = IdentifierEdgeHalf::new(pnt_prev, pnt_next);
    
            if try_get_connection(edge.with_reverse()).is_some() {
                // TODO check neighbouring connection for if we can occlude this edge
                // TODO check for bellow other min-floor, extrude up
                push_edge(edge, &[
                    pnt_prev.extend(surf_curr.get_height_at(pnt_prev)),
                    pnt_prev.extend(surf_prev.get_height_at(pnt_prev)),
                    pnt_next.extend(surf_prev.get_height_at(pnt_next)),
                    pnt_next.extend(surf_curr.get_height_at(pnt_next)),
                ]);
            } else {
                let surf_next = &section.surfaces[part+1]; // SAFE: We're !is_end
                push_edge(edge, &[
                    pnt_prev.extend(surf_curr.get_height_at(pnt_prev)),
                    pnt_prev.extend(surf_next.get_height_at(pnt_prev)),
                    pnt_next.extend(surf_next.get_height_at(pnt_next)),
                    pnt_next.extend(surf_curr.get_height_at(pnt_next)),
                ]);
            }

        }
    } else {
        // TODO Honestly, uhhhh...... yeah.......
    }
}

/*
    /*
            let edge = IdentifierEdgeHalf::new(end, start);
        if let Some(connection) = try_get_connection(edge) {
            let height_other = [
                find_floor_for(height_self[0], start, &connection.surfaces),
                find_floor_for(height_self[1], start, &connection.surfaces),
            ];
        }
     */

fn find_floor_for(height: i16, point: Point2, surfaces: &[Surface]) -> i16 {
    let mut result = height;
    for (i, floor) in surfaces.iter().enumerate().step_by(2) {
        let tmp = floor.get_height_at(point);
        if tmp > height { break; }
        result = tmp;
    }
    result
}
*/