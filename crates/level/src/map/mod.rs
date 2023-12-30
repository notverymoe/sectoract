// Copyright 2023 Natalie Baker // AGPLv3 //

#[macro_use]
mod identifier;

mod angle;
pub use angle::*;

mod connection;
pub use connection::*;

mod point;
pub use point::*;

mod section;
pub use section::*;

mod sector;
pub use sector::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub sectors:     Vec<Sector>,
    pub connections: Vec<Connection>,
}