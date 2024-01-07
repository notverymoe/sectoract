// Copyright 2023 Natalie Baker // AGPLv3 //

use sectoract_level::{util::{SectorBuilder, extract_section_points_from_sector, extract_boundry_from_sector}, map::{Section, Point2, UNIT_WORLD_I, Surface, IdentifierSection, IdentifierEdgeHalf}};

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
                [          0, WIDTH_PLATFORM],
            ])
        )
        .add_section(
            Section::flat_with_roof(0, HEIGHT_MALL), 
            Point2::from_slice_const([
                [          0,                   WIDTH_PLATFORM],
                [LENGTH_MALL,                   WIDTH_PLATFORM],
                [LENGTH_MALL, WIDTH_CROSSOVER + WIDTH_PLATFORM],
                [          0, WIDTH_CROSSOVER + WIDTH_PLATFORM],
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
                [LENGTH_MALL, WIDTH_CROSSOVER +   WIDTH_PLATFORM],
                [LENGTH_MALL, WIDTH_CROSSOVER + 2*WIDTH_PLATFORM],
                [          0, WIDTH_CROSSOVER + 2*WIDTH_PLATFORM],
            ])
        );

    polys_to_svg(sector.edges(), "test_export_dir/out_builder.svg");

    let sector = sector.build().unwrap();
    println!("{:#?}", sector);

    let edges: Vec<_> = sector.graph.keys().map(|v| vec![v.prev(), v.next()].into_boxed_slice()).collect();
    polys_to_svg(edges.iter().map(|v| -> &[Point2] { v }), "test_export_dir/out_graph.svg");

    // // Extract Sector Contours // //
    let mut sector_list: Vec<Vec<Point2>> = Vec::with_capacity(sector.sections.len());
    for section in 0..sector.sections.len() {
        sector_list.push(extract_section_points_from_sector(
            &sector.graph, 
            IdentifierSection::from_raw(section as u16)
        ).unwrap());
    }

    polys_to_svg(sector_list.iter().map(|v| -> &[Point2] { v }), "test_export_dir/out_rebuilt.svg");

    // // Build Surfaces // //

    let mut obj_str = "".to_owned();
    let mut vert_count = 0;
    for (section, points) in sector.sections.iter().zip(sector_list.iter()) {
        for (i, surface) in section.surfaces.iter().enumerate() {
            let Surface::Flat{height} = *surface else { panic!("Slopes are unsupported") };
            
            vert_count = if i % 2 == 0 {
                append_ngon_to_obj_str(&mut obj_str, vert_count, points.iter().map(|&v| v.extend(height)))
            } else {
                append_ngon_to_obj_str(&mut obj_str, vert_count, points.iter().rev().map(|&v| v.extend(height)))
            };

            if i > 0 && (i % 2 == 0) {
                let Surface::Flat{height: height_prev} = section.surfaces[i-1] else { panic!("Slopes are unsupported") };
                for (i, prev) in points.iter().enumerate() {
                    let next = points[(i+1)%points.len()];
                    let edge = IdentifierEdgeHalf::new(*prev, next);

                    // if on boundry, hide edge
                    if sector.graph.contains_key(&edge.with_reverse()) {
                        vert_count = append_ngon_to_obj_str(&mut obj_str, vert_count, [
                            prev.extend(height),
                            prev.extend(height_prev),
                            next.extend(height_prev),
                            next.extend(height),
                        ].into_iter())
                    }
                }
            }

        }
    }

    // TODO move into helper
    let mut boundry: Vec<IdentifierEdgeHalf> = Vec::new();
    let edges = sector.graph.keys().copied().collect::<Vec<_>>().into_boxed_slice();
    for key in sector.graph.keys() {
        let key_rev = key.with_reverse();
        if !sector.graph.contains_key(&key_rev) {
            extract_boundry_from_sector(&mut boundry, key_rev, &edges);
            break;
        }
    }

    for edge in boundry {
        let prev = edge.prev();
        let next = edge.next();

        // Get range for edge
        let section = &sector.sections[sector.graph.get(&edge.with_reverse()).unwrap().section.to_raw() as usize];
        let Some(Surface::Flat{height: height_min}) = section.surfaces.first() else { panic!("Slopes are unsupported") };
        let Some(Surface::Flat{height: height_max}) = section.surfaces.last() else { panic!("Slopes are unsupported") };

        vert_count = append_ngon_to_obj_str(&mut obj_str, vert_count, [
            prev.extend(*height_min),
            next.extend(*height_min),
            next.extend(*height_max),
            prev.extend(*height_max),
        ].into_iter())
    }

    std::fs::write("test_export_dir/out_surfaces.obj", obj_str).unwrap();
}