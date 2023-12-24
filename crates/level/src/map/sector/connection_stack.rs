// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::graphics::TextureFace;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SectorConnectionStack([SectorConnectionStackSide; 2]);

impl SectorConnectionStack {

    #[must_use]
    pub fn new(
        lower: SectorConnectionStackSide,
        upper: SectorConnectionStackSide
    ) -> Self {
        Self([lower, upper])
    }

    #[must_use]
    pub fn lower(&self) -> &SectorConnectionStackSide {
        &self.0[0]
    }

    #[must_use]
    pub fn upper(&self) -> &SectorConnectionStackSide {
        &self.0[1]
    }

    #[must_use]
    pub fn lower_mut(&mut self) -> &mut SectorConnectionStackSide {
        &mut self.0[0]
    }

    #[must_use]
    pub fn upper_mut(&mut self) -> &mut SectorConnectionStackSide {
        &mut self.0[1]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SectorConnectionStackSide {
    pub target: u16,
    pub opaque: bool,
    pub screen: Option<TextureFace>,
}

impl SectorConnectionStackSide {

    #[must_use]
    pub fn new(target: u16) -> Self {
        Self{target, opaque: false, screen: None}
    }

}

