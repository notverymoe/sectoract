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

mod light;
pub use light::*;

#[derive(Debug, Clone, Copy)]
pub struct MaterialTexture {
    pub id:     IdentifierTexture,
    pub offset: TextureOffset,
    pub scale:  TextureScale,
    pub anchor: TextureAnchor,
    pub alpha:  u8,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdentifierTexture(SmolStr);

