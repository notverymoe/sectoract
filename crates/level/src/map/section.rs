// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::Point2;

#[derive(Debug, Default)]
pub struct Section {
    pub surfaces: Vec<Surface>,
}

impl Section {

    #[must_use]
    pub const fn new(surfaces: Vec<Surface>) -> Self {
        Self{surfaces}
    }

    #[must_use]
    pub fn flat(floor: i16) -> Self {
        Self::new(vec![Surface::flat(floor)])
    }

    #[must_use]
    pub fn flat_with_roof(floor: i16, height: i16) -> Self {
        Self::new(vec![Surface::flat(floor), Surface::flat(floor+height)])
    }

}

#[derive(Debug)]
pub enum Surface {
    Flat{
        height: i16,
    },
    Slope{
        start: Point2,
        end:   Point2,
        range: [i16; 2],
    },
}

impl Surface {
    #[must_use]
    pub const fn flat(height: i16) -> Self {
        Self::Flat{height}
    }
    
    #[must_use]
    pub const fn slope(start: Point2, end: Point2, range: [i16; 2]) -> Self {
        Self::Slope{start, end, range}
    }
    
    #[must_use]
    pub const fn flat_2(floor: i16, height: i16) -> [Self; 2] {
        [
            Self::flat(floor),
            Self::flat(floor + height),
        ]
    }
}