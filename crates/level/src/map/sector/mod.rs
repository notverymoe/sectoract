// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::graphics::LightAmbient;

mod anchor_slice_iter;
pub use anchor_slice_iter::*;

mod anchor;
pub use anchor::*;

mod edge;
pub use edge::*;

mod edge_connection;
pub use edge_connection::*;

mod nested;
pub use nested::*;

mod surface;
pub use surface::*;

#[derive(Debug, Clone)]
pub struct Sector {
    pub ambient:  LightAmbient,
    pub edges:    SectorEdges,
    pub surfaces: [SectorSurface; 2],
    pub fluid:    Option<SectorSurface>,
}
