// Copyright 2023 Natalie Baker // AGPLv3 //

use nvm_str_id::SmolStr;

#[macro_use]
mod identifier;

mod angle;
use std::collections::HashMap;

pub use angle::*;

mod point;
pub use point::*;

create_u16_ident!(pub IdentifierNode   );
create_u16_ident!(pub IdentifierSection);

// //

pub struct Sector {
    pub points:   Vec<SectorPoint2>,
    pub nodes:    Vec<SectionNode>,
    pub sections: Vec<Section>,
    pub section_lookup: HashMap<IdentifierNamedSection, IdentifierSection>,
}

// //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierNamedSection(SmolStr);

impl IdentifierNamedSection {

    pub const fn new(name: &str) -> Self {
        Self(SmolStr::new(name))
    }

    pub fn from_raw(value: u128) -> Self {
        Self(SmolStr::from_raw(value))
    }

    pub fn to_raw(&self) -> u128 {
        self.0.to_raw()
    }

}


pub struct Section {
    pub parts: Vec<SectionPart>,
}

pub struct SectionNode {
    pub connections: Vec<SectionNodeConnection>,
}

pub struct SectionNodeConnection {
    pub next:  IdentifierNode,
    pub sides: [IdentifierSection; 2],
}

pub struct SectionPart {
    pub surfaces: [Slope; 2],
}

// //

pub enum Slope {
    Flat{
        height: i16,
    },
    Point{
        start: u16,
        range: [i16; 2],
    },
    Edge{
        start: u16,
        range: [i16; 2],
    }
}