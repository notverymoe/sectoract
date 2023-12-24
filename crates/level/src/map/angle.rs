// Copyright 2023 Natalie Baker // AGPLv3 //

pub fn to_world_angle(v: i16) -> f32 {
    std::f32::consts::TAU/(v as f32)
}

pub fn from_world_angle(v: f32) -> i16 {
    ((std::f32::consts::TAU/v).rem_euclid(i16::MAX as f32)) as i16
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