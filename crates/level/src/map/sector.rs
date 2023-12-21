// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{SectorPoint2, EdgeTexture, SectorPoint3, TextureScaleMode, ColourRGBA};

pub struct Sector {
    pub edge_points:  Box<[SectorPoint2]>,
    pub edge_texture: Box<[EdgeTexture ]>,

    pub surface_upper: SectorSurface,
    pub surface_lower: SectorSurface,
}

pub struct SectorAnchor(u16);

pub struct SectorSurface {
    pub texture: SectorTexture,
    pub slope:   SectorSlope,
}

pub struct SectorSlope {
    pub anchor: SectorAnchor,
    pub start: i32,
    pub end:   i32,
}

// //

pub struct SectorConnection {
    pub idx_sector_outer: u16,
    pub idx_sector_inner: u16,
    pub position: SectorPoint3,
}

// //

#[repr(u8)]
pub enum SectorTextureAnchor {
    Left,
    Right
}

pub struct SectorTexture {
    pub id:      u16,
    pub anchor:  SectorTextureAnchor,
    pub edge:    u16,
    pub offset: [u16; 2],
    pub scale:   TextureScaleMode,
    pub colour:  ColourRGBA,
}