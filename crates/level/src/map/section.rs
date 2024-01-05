// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::SectorAngle;

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
