// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::SectorAngle;

pub struct Section {
    pub parts: Vec<SectionPart>,
}

pub struct SectionPart {
    pub surfaces: [Surface; 2],
}

pub enum Surface {
    Flat{
        height: i16,
    },
    Point{
        start: u16,
        angle: SectorAngle,
        range: [i16; 2],
    },
    Edge{
        start: u16,
        range: [i16; 2],
    }
}
