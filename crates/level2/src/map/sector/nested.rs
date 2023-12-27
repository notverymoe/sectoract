// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{Transform, IdentifierSector};

#[derive(Debug, Clone, Copy)]
pub struct SectorNested {
    pub sector:    IdentifierSector,
    pub transform: Transform,
}