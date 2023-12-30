// Copyright 2023 Natalie Baker // AGPLv3 //

use super::{SectorPoint2, Section};

create_u16_ident!(pub IdentifierMaterial);
create_u16_ident!(pub IdentifierPoint   );
create_u16_ident!(pub IdentifierSector  );
create_u16_ident!(pub IdentifierSection );

#[derive(Debug, Clone)]
pub struct Sector {
    pub points:   Vec<SectorPoint2>,
    pub sections: Vec<Section>,
    pub boundry:  IdentifierSection,
}
