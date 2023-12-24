// Copyright 2023 Natalie Baker // AGPLv3 //

pub struct SectorAnchor(u16);

pub struct SectorSlope {
    pub anchor: SectorAnchor,
    pub start: i32,
    pub end:   i32,
}