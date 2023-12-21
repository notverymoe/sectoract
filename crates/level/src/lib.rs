// Copyright 2023 Natalie Baker // AGPLv3 //

mod units;
pub use units::*;

pub struct Map {
    pub sectors:           Box<[Sector]>,
    pub connection_edge:   Box<[ConnectionEdge]>,
    pub connection_sector: Box<[ConnectionSector]>,
}

// //

pub struct Sector {
    pub points: Box<[SectorPoint2]>,
}

// //

pub struct EdgeReference {
    pub idx_sector: u16,
    pub idx_edge:   u16,
    pub height:     i16,
}

pub enum EdgeConnectionKind {
    Portal,
    Teleport
}

pub struct ConnectionEdge {
    pub kind: EdgeConnectionKind,
    pub sides: [EdgeReference; 2],
}

// //

pub struct ConnectionSector {
    pub idx_sector_outer: u16,
    pub idx_sector_inner: u16,
    pub position: SectorPoint3,
}

// //

pub enum TextureScaleMode {
    FitTexelWidth(u16),
    FitTexelHeight(u16),
}

// //

pub enum TextureEdgeAnchor {
    TpLt,
    TpRt,
    BtLt,
    BtRt,
}

pub struct TextureEdge {
    pub texture: u16,
    pub anchor: TextureEdgeAnchor,
    pub offset: [u16; 2],
    pub scale:  TextureScaleMode,
}

// //

pub enum TextureSectorAnchor {
    Left,
    Right
}

pub struct TextureSector {
    pub texture: u16,
    pub anchor: TextureSectorAnchor,
    pub edge:    u16,
    pub offset: [u16; 2],
    pub scale:  TextureScaleMode,
}