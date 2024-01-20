// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};

#[derive(Default, Resource)]
pub struct CameraReservedSpace {
    pub left:   f32,
    pub right:  f32,
    pub top:    f32,
    pub bottom: f32,
}

impl CameraReservedSpace {

    pub fn position(&self, physical_scale: f32) -> UVec2 {
        UVec2::new(
            (self.left * physical_scale) as u32,
            (self.top  * physical_scale) as u32,
        )
    }

    pub fn size(&self, physical_scale: f32, physical_size: UVec2) -> UVec2 {
        let reserved = UVec2::new(
            ((self.right  + self.left) * physical_scale) as u32,
            ((self.bottom + self.top ) * physical_scale) as u32,
        ).min(physical_size);

        (physical_size - reserved).max(UVec2::ONE)
    }

}

pub fn update_camera_reserved_space(
    mut q_cameras: Query<&mut Camera>,
    q_windows: Query<(Entity, &Window)>,
    q_primary: Query<&Window, With<PrimaryWindow>>,
    r_space: Res<CameraReservedSpace>,
) {
    for mut camera in q_cameras.iter_mut() {

        let scale = match camera.target {
            bevy::render::camera::RenderTarget::Image(_) => 1.0,
            bevy::render::camera::RenderTarget::TextureView(_) => 1.0,
            bevy::render::camera::RenderTarget::Window(w) => match w {
                bevy::window::WindowRef::Primary   => q_primary.single().scale_factor(),
                bevy::window::WindowRef::Entity(e) => q_windows.iter().find(|(v, _)| *v == e).map(|(_, v)| v.scale_factor()).unwrap_or(1.0),
            } 
        } as f32;
        
        camera.viewport = Some(Viewport{
            physical_position: r_space.position(scale),
            physical_size:     r_space.size(scale, camera.physical_target_size().unwrap()),
            ..Default::default()
        })
    }
}