// Copyright 2023 Natalie Baker // AGPLv3 //

use super::{SectorPoint3, SectorAngle};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub origin:      u16,
    pub translation: SectorPoint3,
    pub orentiation: SectorAngle,
}