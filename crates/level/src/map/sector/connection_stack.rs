// Copyright 2023 Natalie Baker // AGPLv3 //

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SectorConnectionStack([u16; 2]);

impl SectorConnectionStack {

    #[must_use]
    pub const fn new(lower: u16, upper: u16) -> Self {
        Self([lower, upper])
    }

    #[must_use]
    pub const fn lower(&self) -> u16 {
        self.0[0]
    }

    #[must_use]
    pub const fn upper(&self) -> u16 {
        self.0[1]
    }

    pub fn set_lower(&mut self, id: u16) {
        self.0[0] = id;
    }

    pub fn set_upper(&mut self, id: u16) {
        self.0[1] = id;
    }
}
