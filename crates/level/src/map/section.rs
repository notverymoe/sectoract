// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::Point2;

use super::{from_world_pos, from_world_pos_m};

#[derive(Debug, Default)]
pub struct Section {
    pub parts: Vec<SectionPart>,
}

#[derive(Debug)]
pub struct SectionPart {
    pub surfaces: [Surface; 2],
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
    pub const fn flat_m(height: i16) -> Self {
        Self::flat(from_world_pos_m(height))
    }

    #[must_use]
    pub fn flat_world(height: f32) -> Self {
        Self::flat(from_world_pos(height))
    }
}

impl Surface {
    #[must_use]
    pub const fn slope(start: Point2, end: Point2, range: [i16; 2]) -> Self {
        Self::Slope{start, end, range}
    }

    #[must_use]
    pub const fn slope_m(start: Point2, end: Point2, range: [i16; 2]) -> Self {
        Self::slope(start, end, [from_world_pos_m(range[0]), from_world_pos_m(range[1])])
    }

    #[must_use]
    pub fn slope_world(start: [f32; 2], end: [f32; 2], range: [f32; 2]) -> Self {
        Self::slope(
            Point2::from_world(start[0], start[1]), 
            Point2::from_world(  end[0],   end[1]), 
            [from_world_pos(range[0]), from_world_pos(range[1])]
        )
    }
}

impl Surface {
    #[must_use]
    pub const fn flat_2(floor: i16, height: i16) -> [Self; 2] {
        [
            Self::flat(floor),
            Self::flat(floor + height),
        ]
    }
}