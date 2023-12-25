// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{
    map::SectorPoint3, 
    graphics::ColourRGBA
};

#[derive(Debug, Clone, Copy)]
pub struct LightAmbient {
    pub intensity: i16,
    pub colour: ColourRGBA,
}

#[derive(Debug, Clone, Copy)]
pub struct LightDirectional {
    pub intensity: i16,
    pub colour:    ColourRGBA,
    pub direction: SectorPoint3,
}