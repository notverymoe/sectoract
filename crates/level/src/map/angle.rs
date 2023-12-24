// Copyright 2023 Natalie Baker // AGPLv3 //

pub fn to_world_angle(v: u16) -> f32 {
    std::f32::consts::TAU/(v as f32)
}

pub fn from_world_angle(v: f32) -> u16 {
    (std::f32::consts::TAU/v).rem_euclid(u16::MAX as f32) as u16
}