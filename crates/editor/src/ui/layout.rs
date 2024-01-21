// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{
    prelude::*, 
    render::camera::Viewport, 
    window::PrimaryWindow
};

#[derive(Default)]
pub struct ReservedSpace {
    pub left:   u32,
    pub right:  u32,
    pub top:    u32,
    pub bottom: u32,
}

impl ReservedSpace {
    pub fn viewport_origin(&self) -> UVec2 {
        UVec2::new(self.left, self.top)
    }

    pub fn viewport_size(&self, size: UVec2) -> UVec2 {
        let reserved = UVec2::new(
            self.left + self.right,
            self.top  + self.bottom,
        );

        (size - reserved).max(UVec2::ONE)
    }
}

pub fn update_camera_reserved_space(
    mut q_cameras: Query<&mut Camera>,
    r_layout: Res<ScreenLayout>,
) {
    for mut camera in q_cameras.iter_mut() {
        if let Some(physical_size) = camera.physical_target_size() {
            camera.viewport = Some(r_layout.get_physical_viewport(physical_size));
        }
    }
}

#[derive(Resource)]
pub struct ScreenLayout {
    pub scale: f32,
    pub space: ReservedSpace,
}

impl Default for ScreenLayout {
    fn default() -> Self {
        Self { 
            scale: 1.0, 
            space: Default::default(),
        }
    }
}

impl ScreenLayout {

    pub fn to_physical(&self, logical: Vec2) -> UVec2 {
        (logical * self.scale).as_uvec2()
    }

    pub fn to_logical(&self, physical: UVec2) -> Vec2 {
        physical.as_vec2() / self.scale
    }

    pub fn get_physical_viewport(&self, size_physical: UVec2) -> Viewport {
        Viewport{
            physical_position: self.space.viewport_origin(),
            physical_size:     self.space.viewport_size(size_physical),
            ..Default::default()
        }
    }

}

pub fn update_physical_scale(
    mut r_layout: ResMut<ScreenLayout>,
    q_primary: Query<&Window, With<PrimaryWindow>>,
) {
    r_layout.scale = q_primary.single().scale_factor() as f32;
}
