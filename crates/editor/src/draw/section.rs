// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{gizmos::gizmos::Gizmos, render::color::Color, prelude::Vec2};
use sectoract_level::map::Sector;

pub fn section_lines(
    gizmos: &mut Gizmos, 
    sector: &Sector,
    colour_passable:   Color,
    colour_impassable: Color, 
) {
    for edge in sector.graph.keys() {
        let is_border = sector.graph.contains_key(&edge.with_reverse());
        gizmos.line_2d(
            Vec2::from_array(edge.prev().to_world()), 
            Vec2::from_array(edge.next().to_world()), 
            if is_border { colour_impassable } else { colour_passable }
        );
    }
}