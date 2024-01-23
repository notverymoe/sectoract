// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{gizmos::gizmos::Gizmos, render::color::Color, math::Vec2};

pub fn grid_simple(gizmos: &mut Gizmos, from: Vec2, to: Vec2, size: Vec2, colour: Color) {
    let x_diff = to.x - from.x;
    let x_iter = (x_diff.abs()/size.x).floor() as usize;
    for x in 0..x_iter {
        let x = from.x + (x as f32)*size.x;
        gizmos.line_2d(Vec2::new(x, from.y), Vec2::new(x, to.y), colour);
    }

    let y_diff = to.y - from.y;
    let y_iter = (y_diff.abs()/size.y).floor() as usize;
    for y in 0..y_iter {
        let y = from.y + (y as f32)*size.y;
        gizmos.line_2d(Vec2::new(from.x, y), Vec2::new(to.x, y), colour);
    }
}

pub fn grid_snapped(gizmos: &mut Gizmos, offset: Vec2, from: Vec2, to: Vec2, size: Vec2, colour: Color) {
    let from = offset + ((from - offset)/size).floor()*size;
    let to   = offset + ((  to - offset)/size).ceil() *size;
    grid_simple(gizmos, from, to, size, colour);
}