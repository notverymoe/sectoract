// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{
    math::{IVec2, Vec2, Vec3}, 
    ecs::{
        system::{Resource, Query, Res, ResMut}, 
        query::With
    }, 
    window::{Window, PrimaryWindow}, 
    render::camera::{Camera, CameraProjection, OrthographicProjection}, 
    transform::components::GlobalTransform
};

use super::ScreenLayout;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum GridSize {
    Scale01 =    1,
    Scale02 =    2,
    Scale03 =    4,
    Scale04 =   10,
    Scale05 =   20,
    #[default]
    Scale06 =   40,
    Scale07 =  100,
    Scale08 =  400,
    Scale09 = 1000,
    Scale10 = 2000,
    Scale11 = 4000,
}

impl GridSize {

    pub fn change(&mut self, delta: i32) {
        *self = Self::from_index(self.to_index() + delta);
    }

    #[must_use]
    pub fn snap_to(self, pos: Vec2) -> IVec2 {
        (pos / (self.get_units() as f32)).round().as_ivec2() * (self.get_units() as i32)
    }

    #[must_use]
    pub fn snap_to_ceil(self, pos: Vec2) -> IVec2 {
        (pos / (self.get_units() as f32)).ceil().as_ivec2() * (self.get_units() as i32)
    }

    #[must_use]
    pub fn snap_to_floor(self, pos: Vec2) -> IVec2 {
        (pos / (self.get_units() as f32)).floor().as_ivec2() * (self.get_units() as i32)
    }

    #[must_use]
    pub fn snap_to_bounds(self, pos: Vec2, min: Vec2, max: Vec2) -> IVec2 {
        let pos = self.snap_to(pos);
        let min = self.snap_to_ceil(min);
        let max = self.snap_to_floor(max);
        pos.max(min).min(max)
    }

    #[must_use]
    pub const fn get_units(self) -> u32 {
        self as u32
    }

    #[must_use]
    pub const fn to_index(self) -> i32 {
        match self {
            GridSize::Scale01 =>  0,
            GridSize::Scale02 =>  1,
            GridSize::Scale03 =>  2,
            GridSize::Scale04 =>  3,
            GridSize::Scale05 =>  4,
            GridSize::Scale06 =>  5,
            GridSize::Scale07 =>  6,
            GridSize::Scale08 =>  7,
            GridSize::Scale09 =>  8,
            GridSize::Scale10 =>  9,
            GridSize::Scale11 => 10,
        }
    }

    #[must_use]
    pub fn from_index(v: i32) -> Self {
        match v.max(0).min(10) {
            0 => GridSize::Scale01,
            1 => GridSize::Scale02,
            2 => GridSize::Scale03,
            3 => GridSize::Scale04,
            4 => GridSize::Scale05,
            5 => GridSize::Scale06,
            6 => GridSize::Scale07,
            7 => GridSize::Scale08,
            8 => GridSize::Scale09,
            9 => GridSize::Scale10,
           10 => GridSize::Scale11,
           _  => unreachable!(),
        }
    }

}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource)]
pub struct GridSettings {
    pub size: GridSize,
}

#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct GridCursor {
    pub is_active: bool,
    pub pos_viewport: Vec2,
    pub pos_world: Vec2,
    pub pos_grid:  IVec2,
    pub norm_viewport: Vec2,
    pub delta_viewport: Vec2,
    pub delta_viewport_scaled: Vec2,
}

impl GridCursor {

    #[must_use]
    pub fn update_from_camera(
        self,
        pos_cursor_logical: Vec2,
        camera: &Camera,
        camera_transform: &GlobalTransform,
        grid_settings:    &GridSettings,
        layout:           &ScreenLayout,
        projection: &OrthographicProjection
    ) -> Self {
        let viewport_size   = camera.logical_viewport_size().unwrap_or(Vec2::ONE);
        let viewport_offset = projection.viewport_origin * viewport_size;

        let pos_viewport = pos_cursor_logical - layout.to_logical(layout.space.viewport_origin());
        let pos_min = camera_transform.translation().truncate() - viewport_offset*projection.scale;
        let pos_max = pos_min + viewport_size*projection.scale;

        let pos_world = camera.viewport_to_world_2d(camera_transform, pos_viewport).unwrap_or(Vec2::ZERO).max(pos_min).min(pos_max);
        let pos_grid  = grid_settings.size.snap_to_bounds(pos_world, pos_min, pos_max);

        let norm_viewport = pos_viewport / viewport_size;

        let delta_viewport = pos_viewport - self.pos_viewport;

        let delta_viewport_norm = delta_viewport / camera.logical_viewport_size().unwrap();

        let delta_viewport_scaled = Vec2::new(
             delta_viewport_norm.x * projection.area.width(),
            -delta_viewport_norm.y * projection.area.height(),
        );

        GridCursor{
            is_active: true,
            pos_viewport,
            pos_world,
            pos_grid,
            norm_viewport,
            delta_viewport,
            delta_viewport_scaled,
        }
    }
}

pub fn update_grid_cursor(
    mut r_cursor: ResMut<GridCursor>,
    r_grid_settings: Res<GridSettings>,
    r_layout: Res<ScreenLayout>,
    q_cameras: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single();
    // TODO (?) pin cursor pos to closest edge
    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, camera_proj, camera_transform) = q_cameras.single();
        *r_cursor = r_cursor.update_from_camera(cursor_pos, camera, camera_transform, &r_grid_settings, &r_layout, camera_proj);
    } else {
        r_cursor.is_active = false;
    }
}