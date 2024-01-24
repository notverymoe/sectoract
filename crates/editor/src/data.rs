use sectoract_level::{map::{Section, Sector, Surface, UNIT_WORLD_I, Point2}, util::SectorBuilder};

pub const HEIGHT_MALL:     i16 = UNIT_WORLD_I * 22;
pub const LENGTH_MALL:     i16 = UNIT_WORLD_I * 36;
pub const WIDTH_PLATFORM:  i16 = UNIT_WORLD_I * 3;
pub const WIDTH_CROSSOVER: i16 = UNIT_WORLD_I * 6;

#[allow(too_many_lines)]
#[must_use]
pub fn get_test_sector() -> Sector {
    SectorBuilder::new()
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
                ).build().unwrap()
}