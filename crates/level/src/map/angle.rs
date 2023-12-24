// Copyright 2023 Natalie Baker // AGPLv3 //

use core::f32::consts as f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SectorAngle(i16);

impl SectorAngle {

    #[must_use]
    pub fn new(v: i16) -> Self {
        Self(v)
    }

    #[must_use]
    pub fn from_radians(v: f32) -> Self {
        Self::new(from_world_angle(v))
    }

    #[must_use]
    pub fn from_degrees(v: f32) -> Self {
        Self::new(from_world_angle(v.to_radians()))
    }

    #[must_use]
    pub fn to_radians(&self) -> f32 {
        to_world_angle(self.0)
    }

    #[must_use]
    pub fn to_degrees(&self) -> f32 {
        to_world_angle(self.0).to_degrees()
    }

}

#[must_use]
pub fn to_world_angle(v: i16) -> f32 {
    f32::TAU/(v as f32)
}

#[must_use]
pub fn from_world_angle(v: f32) -> i16 {
    ((f32::TAU/v).rem_euclid(i16::MAX as f32)) as i16
}

#[cfg(test)]
mod tests {
    use super::{from_world_angle, to_world_angle};

    #[test]
    fn round_trip() {
        let do_test = |v: i16| assert_eq!(v.rem_euclid(i16::MAX), from_world_angle(to_world_angle(v)));

        do_test(0);

        do_test(-1);
        do_test( 1);

        do_test(i16::MAX);
        do_test(i16::MIN);

        do_test(i16::MAX-1);
        do_test(i16::MIN+1);
    }

}