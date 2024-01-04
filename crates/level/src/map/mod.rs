// Copyright 2023 Natalie Baker // AGPLv3 //

use core::fmt::Debug;

#[macro_use]
mod identifier;

mod angle;
pub use angle::*;

mod point;
pub use point::*;

mod sector;
pub use sector::*;

mod edge_half_identifier;
pub use edge_half_identifier::*;

mod section;
pub use section::*;

create_ident!(u16, pub IdentifierSectorPoint);
create_ident!(u16, pub IdentifierSectorEdge );

create_ident!(u16, pub IdentifierSection    );
create_ident!(u16, pub IdentifierSectionPart);

pub struct IdentifierSectionEdge {
    pub edge: IdentifierEdgeHalf,
    pub part: IdentifierSectionPart,
}