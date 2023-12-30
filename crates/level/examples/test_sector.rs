// Copyright 2023 Natalie Baker // AGPLv3 //

use sectoract_level::map::{Sector, SectorPoint2, Section, IdentifierPoint, IdentifierSection, SectionSlope};

pub fn main() {
    let sector = Sector{
        points: vec![
            // boundry L-shape         // |--|:1:|:2:|:3:|:4:|
            SectorPoint2::new( 0,  0), // | 0| 0 | - | - | - |
            SectorPoint2::new(10,  0), // | 1| - | 0 | - | - |
            SectorPoint2::new( 5,  5), // | 2| 1 | 1 | - | - |
            SectorPoint2::new(10,  5), // | 3| - | 2 | - | - |
            SectorPoint2::new( 5,  5), // | 4| 2 | 3 | - | 1 |
            SectorPoint2::new( 5, 10), // | 5| - | - | 4 | - |
            SectorPoint2::new( 0, 10), // | 6| - | - | 5 | - |
            SectorPoint2::new( 0,  5), // | 7| 3 | - | 0 | - |
            // Section 3 / 4           // |--|:1:|:2:|:3:|:4:|
            SectorPoint2::new( 3,  5), // | 8| - | - | 1 | 0 |
            SectorPoint2::new( 3,  7), // | 9| - | - | 2 | 3 |
            SectorPoint2::new( 5,  7), // |10| - | - | 3 | 2 |
            // Section 5 / 6           // |--|:5:|:6:|:7:|: :|
            SectorPoint2::new( 6,  1), // |11| - | 0 | - |   |
            SectorPoint2::new( 8,  1), // |12| - | 1 | 0 |   |
            SectorPoint2::new( 8,  3), // |13| 1 | 2 | 5 |   |
            SectorPoint2::new( 8,  4), // |14| 2 | - | 4 |   |
            SectorPoint2::new( 6,  4), // |15| 3 | - | - |   |
            SectorPoint2::new( 6,  3), // |16| 0 | 3 | - |   |
            SectorPoint2::new( 9,  1), // |17| - | - | 1 |   |
            SectorPoint2::new( 9,  3), // |18| - | - | 2 |   |
            SectorPoint2::new( 9,  4), // |19| - | - | 3 |   |
        ],
        sections: vec![
            // boundry
            Section{
                parent:   None,
                surfaces: None,
                edges: vec![
                    IdentifierPoint::from(0),
                    IdentifierPoint::from(1),
                    IdentifierPoint::from(2),
                    IdentifierPoint::from(3),
                    IdentifierPoint::from(4),
                    IdentifierPoint::from(5),
                    IdentifierPoint::from(6),
                    IdentifierPoint::from(7),
                ]
            },
            // section 1
            Section{
                parent:   None,
                surfaces: Some([
                    SectionSlope::flat(4,  1),
                    SectionSlope::flat(4, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from(0),
                    IdentifierPoint::from(2),
                    IdentifierPoint::from(4),
                    IdentifierPoint::from(7),
                ]
            },
            // section 2
            Section{
                parent:   None,
                surfaces: Some([
                    SectionSlope::flat(4,  1),
                    SectionSlope::flat(4, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from(1),
                    IdentifierPoint::from(2),
                    IdentifierPoint::from(3),
                    IdentifierPoint::from(4),
                ]
            },
            // section 3
            Section{
                parent:   None,
                surfaces: Some([
                    SectionSlope::flat(6,  3),
                    SectionSlope::flat(6, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from( 7),
                    IdentifierPoint::from( 8),
                    IdentifierPoint::from( 9),
                    IdentifierPoint::from(10),
                    IdentifierPoint::from( 5),
                    IdentifierPoint::from( 6),
                ]
            },
            // section 4
            Section{
                parent:   None,
                surfaces: Some([
                    SectionSlope::slope_from_edge(0, vec![1, 3]),
                    SectionSlope::flat(4, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from( 8),
                    IdentifierPoint::from( 4),
                    IdentifierPoint::from(10),
                    IdentifierPoint::from( 9),
                ]
            },
            // section 5
            Section{
                parent:   Some(IdentifierSection::from(2)),
                surfaces: Some([
                    SectionSlope::slope_from_edge(2, vec![1, 0]),
                    SectionSlope::flat(4, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from(16),
                    IdentifierPoint::from(13),
                    IdentifierPoint::from(14),
                    IdentifierPoint::from(15),
                ]
            },
            // section 6
            Section{
                parent:   Some(IdentifierSection::from(2)),
                surfaces: Some([
                    SectionSlope::flat(4,  0),
                    SectionSlope::flat(4, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from(11),
                    IdentifierPoint::from(12),
                    IdentifierPoint::from(13),
                    IdentifierPoint::from(16),
                ]
            },
            // section 7
            Section{
                parent:   Some(IdentifierSection::from(2)),
                surfaces: Some([
                    SectionSlope::slope_from_edge(2, vec![1, 0, 0]),
                    SectionSlope::flat(6, 10),
                ]),
                edges: vec![
                    IdentifierPoint::from(12),
                    IdentifierPoint::from(17),
                    IdentifierPoint::from(18),
                    IdentifierPoint::from(19),
                    IdentifierPoint::from(14),
                    IdentifierPoint::from(13),
                ]
            }
        ],
        boundry: IdentifierSection::from(0),
    };

    let mut polygons: Vec<Vec<[f32; 3]>> = Vec::default();

    for section in sector.sections.iter() {

    }

    println!("{:#?}", sector);
}