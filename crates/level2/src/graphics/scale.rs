// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::graphics::UNIT_TEXTURE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureScale {
    FitTexelWidth(u16),
    FitTexelHeight(u16),
}

impl Default for TextureScale {
    fn default() -> Self {
        Self::FitTexelWidth(UNIT_TEXTURE as u16)
    }
}
