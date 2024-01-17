// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{IdentifierEdgeHalf, Point3};

mod section;
pub use section::*;

mod sector;
pub use sector::*;

pub trait FaceWriter {
    fn add_surf(&mut self, section: usize, part: usize, face: impl IntoIterator<Item = Point3>);
    fn add_wall(&mut self, section: usize, part: usize, edge: IdentifierEdgeHalf, face: impl IntoIterator<Item = Point3>);
}