// Copyright 2023 Natalie Baker // AGPLv3 //

mod angle;
pub use angle::*;

mod connection;
pub use connection::*;

#[macro_use]
mod identifier;
pub use identifier::*;

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