// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Debug, Default, Component)]
pub struct PancamViewport {
    center:        Vec2,
    size_unscaled: Vec2,
    scale:         f32,
}

impl PancamViewport {
    pub fn pan(&mut self, delta: Vec2) {
        self.center += delta;
    }

    pub fn zoom_towards(&mut self, target: Vec2, zoom: f32) {        
        let scale_new = (self.scale.ln() + zoom).exp();
        let scale_ratio = scale_new/self.scale;
        self.center = self.center + (1.0 - scale_ratio)*(target - self.center);
        self.scale  = scale_new;
    }
}

#[derive(Debug, Default, Component)]
pub struct PancamCursor {
    pub world_pos:      Option<Vec2>,
    pub viewport_pos:   Option<Vec2>,
    pub viewport_delta: Option<Vec2>,
}

fn extract_pancam_viewport(
    mut q_cameras: Query<(&Camera, &OrthographicProjection, &GlobalTransform, &mut PancamViewport)>,
) {
    for (camera, projection, transform, mut viewport_pan) in &mut q_cameras {
        let viewport_size = camera.logical_viewport_size().unwrap();
        viewport_pan.center       = camera.viewport_to_world_2d(transform, viewport_size*0.5).unwrap();
        viewport_pan.scale        = projection.scale;
        viewport_pan.size_unscaled = ((2.0*(camera.viewport_to_world_2d(transform, viewport_size).unwrap() - viewport_pan.center))/projection.scale).abs();
    }
}

fn extract_pancam_cursor(
    mut q_cameras: Query<(&Camera, &GlobalTransform,&mut PancamCursor)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();
    let window_cursor = window.cursor_position();

    for (camera, transform, mut cursor_pan) in &mut q_cameras {
        let viewport_origin = camera.viewport.as_ref().and_then(|v| camera.to_logical(v.physical_position)).unwrap_or(Vec2::ZERO);
        let viewport_cursor = window_cursor.map(|v| v - viewport_origin);

        cursor_pan.viewport_delta = viewport_cursor.and_then(|n| cursor_pan.viewport_pos.map(|p| p - n));
        cursor_pan.viewport_pos   = viewport_cursor;
        cursor_pan.world_pos      = viewport_cursor.and_then(|v| camera.viewport_to_world_2d(transform, v));
    }
}

fn apply_pancam_viewport(
    mut q_cameras: Query<(&mut PancamViewport, &Camera, &GlobalTransform, &mut Transform, &mut OrthographicProjection)>,
) {
    for (viewport_pan, camera, transform, mut transform_local, mut projection) in &mut q_cameras {
        let viewport_size_old = camera.logical_viewport_size().unwrap_or(Vec2::ONE);
        let world_center = camera.viewport_to_world_2d(transform, viewport_size_old*0.5).unwrap();

        projection.scale = viewport_pan.scale;
        transform_local.translation += (viewport_pan.center - world_center).extend(0.0);
    }
}

pub struct PluginPancam;

impl Plugin for PluginPancam {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, (extract_pancam_cursor, extract_pancam_viewport))
            .add_systems(PostUpdate, apply_pancam_viewport);
    }
}