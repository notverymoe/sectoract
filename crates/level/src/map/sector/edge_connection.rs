// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{IdentifierMaterial, IdentifierEdge};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct EdgeConnection([EdgeConnectionSide; 2]);

#[derive(Debug, Clone, Copy)]
pub struct EdgeConnectionSide {
    pub target: IdentifierEdge,
    pub height: i16,
    pub screen: Option<IdentifierMaterial>,
}
