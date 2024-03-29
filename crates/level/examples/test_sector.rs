// Copyright 2023 Natalie Baker // AGPLv3 //

use sectoract_level::{
    util::SectorBuilder, 
    map::{Section, Point2, UNIT_WORLD_I, Surface}, 
    geo::{build_section_faces, extract_section_contour}
};
use util::ObjFaceWriter;

use crate::util::polys_to_svg;

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
                

                Surface::flat(HEIGHT_MALL + 40),
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
    for section in sector.iter_section_ids() {
        section_pnts.push(extract_section_contour(&sector.graph, section).unwrap());
    }

    polys_to_svg(section_pnts.iter().map(|v| -> &[Point2] { v }), "test_export_dir/out_rebuilt.svg");

    // // Build Surfaces // //

    let mut obj = ObjFaceWriter::new("out_surfaces", sector.sections.len(), sector.sections.iter().map(|v| v.surfaces.len()).max().unwrap_or(1));
    for (section, points) in (0..sector.sections.len()).zip(section_pnts.iter()) {
        build_section_faces(section, &sector.sections, points, |e| sector.graph.get(&e).map(|v| &sector.sections[usize::from(v.section)]), &mut obj);
    }
    obj.write("test_export_dir/").unwrap();
}
