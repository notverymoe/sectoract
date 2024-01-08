// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::map::Point2;

#[derive(Debug, Default)]
pub struct Section {
    pub surfaces: Vec<Surface>,
}

impl Section {

    #[must_use]
    pub const fn new(surfaces: Vec<Surface>) -> Self {
        Self{surfaces}
    }

    #[must_use]
    pub fn flat(floor: i16) -> Self {
        Self::new(vec![Surface::flat(floor)])
    }

    #[must_use]
    pub fn flat_with_roof(floor: i16, height: i16) -> Self {
        Self::new(vec![Surface::flat(floor), Surface::flat(floor+height)])
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Flat{
        height: i16,
    },
    Slope(Slope),
}

impl Surface {
    #[must_use]
    pub const fn flat(height: i16) -> Self {
        Self::Flat{height}
    }
    
    #[must_use]
    pub const fn slope(start: Point2, delta: Point2, range: [i16; 2]) -> Self {
        Self::Slope(Slope{start, delta, range})
    }
    
    #[must_use]
    pub const fn flat_2(floor: i16, height: i16) -> [Self; 2] {
        [
            Self::flat(floor),
            Self::flat(floor + height),
        ]
    }
}

impl Surface {

    #[must_use]
    pub fn get_height_at(&self, point: Point2) -> i16 {
        match self {
            Surface::Flat { height } => *height,
            Surface::Slope( s      ) => s.get_height_at(point),
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub struct Slope {
    pub start: Point2,
    pub delta: Point2,
    pub range: [i16; 2],
}

impl Slope {

    #[must_use]
    pub fn get_height_at(&self, point: Point2) -> i16 {

        let Point2{x: off_x, y: off_y} = point - self.start;

        let off_x = off_x as f32;
        let off_y = off_y as f32;

        let delta_x = self.delta.x as f32;
        let delta_y = self.delta.y as f32;
        let delta_len = ((delta_x*delta_x) + (delta_y*delta_y)).sqrt();

        let norm_x = delta_x/delta_len;
        let norm_y = delta_y/delta_len;

        let off_scl = ((off_x*norm_x) + (off_y*norm_y))/delta_len;

        let range_delta = (self.range[1] - self.range[0]) as f32;

        self.range[0] + (range_delta * off_scl).round() as i16
    }

}