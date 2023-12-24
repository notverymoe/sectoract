// Copyright 2023 Natalie Baker // AGPLv3 //

use nvm_str_id::SmolStr;

mod anchor;
pub use anchor::*;

mod colour;
pub use colour::*;

mod offset;
pub use offset::*;

mod scale;
pub use scale::*;

#[derive(Debug, Clone, Copy)]
pub struct TextureFace {
    pub id:     TextureID,
    pub offset: TextureOffset,
    pub scale:  TextureScale,
    pub anchor: TextureAnchor,
    pub alpha:  u8,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureID(SmolStr);

