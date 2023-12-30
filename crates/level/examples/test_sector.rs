// Copyright 2023 Natalie Baker // AGPLv3 //

use sectoract_level::map::{Sector, SectorPoint2, Section, IdentifierPoint, IdentifierSection, SectionSlope};

mod util;

pub fn main() {
    let sector = Sector{
        points: vec![
            // boundry L-shape                    // |--|:1:|:2:|:3:|:4:|
            SectorPoint2::from_world( 0.0,  0.0), // | 0| 0 | - | - | - |
            SectorPoint2::from_world( 5.0,  0.0), // | 1| 1 | 1 | - | - |
            SectorPoint2::from_world(10.0,  0.0), // | 2| - | 0 | - | - |
            SectorPoint2::from_world(10.0,  5.0), // | 3| - | 2 | - | - |
            SectorPoint2::from_world( 5.0,  5.0), // | 4| 2 | 3 | - | 1 |
            SectorPoint2::from_world( 5.0, 10.0), // | 5| - | - | 4 | - |
            SectorPoint2::from_world( 0.0, 10.0), // | 6| - | - | 5 | - |
            SectorPoint2::from_world( 0.0,  5.0), // | 7| 3 | - | 0 | - |
            // Section 3 / 4                      // |--|:1:|:2:|:3:|:4:|
            SectorPoint2::from_world( 3.0,  5.0), // | 8| - | - | 1 | 0 |
            SectorPoint2::from_world( 3.0,  7.0), // | 9| - | - | 2 | 3 |
            SectorPoint2::from_world( 5.0,  7.0), // |10| - | - | 3 | 2 |
            // Section 5 / 6 / 7                  // |--|:5:|:6:|:7:|: :|
            SectorPoint2::from_world( 6.0,  1.0), // |11| - | 0 | - |   |
            SectorPoint2::from_world( 8.0,  1.0), // |12| - | 1 | 0 |   |
            SectorPoint2::from_world( 8.0,  3.0), // |13| 1 | 2 | 5 |   |
            SectorPoint2::from_world( 8.0,  4.0), // |14| 2 | - | 4 |   |
            SectorPoint2::from_world( 6.0,  4.0), // |15| 3 | - | - |   |
            SectorPoint2::from_world( 6.0,  3.0), // |16| 0 | 3 | - |   |
            SectorPoint2::from_world( 9.0,  1.0), // |17| - | - | 1 |   |
            SectorPoint2::from_world( 9.0,  3.0), // |18| - | - | 2 |   |
            SectorPoint2::from_world( 9.0,  4.0), // |19| - | - | 3 |   |
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
                    IdentifierPoint::from(1),
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
    
    util::sector_to_svg(&sector, "out.svg");
}