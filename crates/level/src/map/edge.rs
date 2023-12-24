// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::graphics::TextureFace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeReference {
    pub idx_sector: u16,
    pub idx_edge:   u16,
}

#[repr(transparent)]
pub struct EdgeConnection([EdgeConnectionSide; 2]);

#[derive(Debug, Clone, Copy)]
pub struct EdgeConnectionSide {
    pub target: EdgeReference,
    pub opaque: bool,
    pub height: i16,
    pub screen: Option<TextureFace>,
    pub pillar: Option<TextureFace>,
}
