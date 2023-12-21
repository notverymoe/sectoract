// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{TextureScaleMode, ColourRGBA};

pub struct EdgeReference {
    pub idx_sector: u16,
    pub idx_edge:   u16,
    pub height:     i16,
}

pub enum EdgeConnectionKind {
    Portal,
    Teleport
}

pub struct EdgeConnection {
    pub kind:  EdgeConnectionKind,
    pub sides: [EdgeReference; 2],
}

// //

#[repr(u8)]
pub enum EdgeTextureAnchor {
    TpLt,
    TpRt,
    BtLt,
    BtRt,
}

pub struct EdgeTexture {
    pub id:      u16,
    pub anchor:  EdgeTextureAnchor,
    pub offset: [u16; 2],
    pub scale:   TextureScaleMode,
    pub colour:  ColourRGBA,
}