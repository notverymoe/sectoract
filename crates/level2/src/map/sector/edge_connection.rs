// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{IdentifierMaterial, IdentifierEdge, IdentifierSector};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct EdgeConnection([EdgeConnectionSide; 2]);

impl EdgeConnection {

    pub fn sides(&self) -> &[EdgeConnectionSide; 2] {
        &self.0
    }

    pub fn side_targeting_sector(&self, id: IdentifierSector) -> Option<u16> {
        if self.0[0].target.sector() == id {
            Some(0)
        } else if self.0[1].target.sector() == id {
            Some(1)
        } else {
            None
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub struct EdgeConnectionSide {
    pub target: IdentifierEdge,
    pub height: i16,
    pub screen: Option<IdentifierMaterial>,
}
