// Copyright 2023 Natalie Baker // AGPLv3 //

use super::{IdentifierSector, IdentifierSection};

#[derive(Debug, Clone, Copy)]
pub struct Connection {
    pub sides: [ConnectionSide; 2]
}

#[derive(Debug, Clone, Copy)]
pub struct ConnectionSide {
    pub target: EdgeConnectionTarget,
    pub height: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeConnectionTarget {
    pub sector:  IdentifierSector,
    pub section: IdentifierSection,
    pub edge:    u16,
}