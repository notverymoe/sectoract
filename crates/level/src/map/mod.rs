// Copyright 2023 Natalie Baker // AGPLv3 //

mod edge;
pub use edge::*;

mod sector;
pub use sector::*;

pub struct Map {
    pub sectors:           Box<[Sector]>,
    pub connection_edge:   Box<[EdgeConnection]>,
    pub connection_sector: Box<[SectorConnection]>,
}

// //

pub enum TextureScaleMode {
    FitTexelWidth(u16),
    FitTexelHeight(u16),
}