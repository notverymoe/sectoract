// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{SectorPoint3, SectorAngle};

#[derive(Debug, Clone, Copy)]
pub struct SectorConnectionNested {
    pub source: u16,
    pub target: u16,
    pub origin: SectorPoint3,
    pub angle:  SectorAngle,
}