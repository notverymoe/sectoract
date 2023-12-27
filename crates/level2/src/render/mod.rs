// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::{IdentifierMaterial, IdentifierSector, IdentifierConnection, EdgeConnection, Sector};

pub struct SectorCache {

    pub surfaces:    [Box<[i16]>; 2],
    pub connections: Vec<Vec<(u16, IdentifierConnection)>>,
}

impl SectorCache {

    #[must_use]
    pub fn new(id: IdentifierSector, sector: &Sector, connections: &[EdgeConnection]) -> Self {
        let edge_len = sector.edges.shape.len();
        Self { 
            surfaces: sector.surfaces.map(|v| {
                let mut surface = vec![0; edge_len].into_boxed_slice();
                assert!(v.resolve_heights(&mut surface));
                surface
            }),
            connections: Self::find_sector_connections(id, edge_len, connections), 
        }
    }

    fn find_sector_connections(id: IdentifierSector, edge_count: usize, connections: &[EdgeConnection]) -> Vec<Vec<(u16, IdentifierConnection)>> {
        let mut connections_to_sector: Vec<Vec<(u16, IdentifierConnection)>> = Vec::new();
        connections_to_sector.resize(edge_count, Vec::default());
        for (edge, side, connection) in connections.iter().enumerate().flat_map(map_sides_targeting_sector(id)) {
            connections_to_sector[edge as usize].push((side, connection));
        }
        connections_to_sector
    }

}

#[derive(Debug, Clone, Copy)]
pub struct GeometryVertex {
    pub point:         [f32; 3],
    pub texture_coord: [f32; 2],
    pub material:      IdentifierMaterial,
}

pub struct WallVertexVisitor {

}


fn map_sides_targeting_sector(id: IdentifierSector) -> impl for<'a> Fn((usize, &'a EdgeConnection)) -> std::option::Option<(u16, u16, IdentifierConnection)> {
    return move |(i, v): (usize, &EdgeConnection)| {
        v.side_targeting_sector(id)
            .map(|s| (
                v.sides()[s as usize].target.edge(), 
                s, 
                IdentifierConnection::new(i as u16)
            ))
    };
}