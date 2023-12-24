// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::graphics::TextureFace;

pub struct EdgeReference {
    pub idx_sector: u16,
    pub idx_edge:   u16,
}

pub enum EdgeConnectionKind {
    Portal,
    Teleport
}

pub struct EdgeConnection {
    pub kind:  EdgeConnectionKind,
    pub sides: [EdgeReference; 2],
}

pub struct EdgeConnectionSide {
    pub id:     EdgeReference,
    pub height: i16,
    pub texture: Option<TextureFace>,
}
