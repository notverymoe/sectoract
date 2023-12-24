// Copyright 2023 Natalie Baker // AGPLv3 //

mod angle;
pub use angle::*;

mod point;
pub use point::*;

mod edge;
pub use edge::*;

mod sector;
pub use sector::*;

pub struct Map {
    pub sectors:           Box<[Sector]>,
    pub connection_edge:   Box<[EdgeConnection]>,
    pub connection_sector: Box<[SectorConnection]>,
}
