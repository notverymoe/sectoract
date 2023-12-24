// Copyright 2023 Natalie Baker // AGPLv3 //

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TextureAnchor {
    #[default]
    BottomLeft,
    TopLeft,
    BottomRight,
    TopRight,
}