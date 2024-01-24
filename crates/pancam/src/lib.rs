// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Debug, Component)]
pub struct PancamSettings {

}

#[derive(Debug, Component)]
pub struct PancamViewport {
    pub world_center: Vec2,
    pub world_size:   Vec2,
}

#[derive(Debug, Component)]
pub struct PancamCursor {
    pub world_pos:      Option<Vec2>,
    pub viewport_pos:   Option<Vec2>,
    pub viewport_delta: Option<Vec2>,
}

fn extract_pancam_viewport(
    mut q_cameras: Query<(&Camera, &GlobalTransform, &mut PancamViewport)>,
) {
    for (camera, transform, mut viewport_pan) in &mut q_cameras {
        let viewport_size = camera.logical_viewport_size().unwrap();
        viewport_pan.world_center = camera.viewport_to_world_2d(transform, viewport_size*0.5).unwrap();
        viewport_pan.world_size   = 2.0*(camera.viewport_to_world_2d(transform, viewport_size).unwrap() - viewport_pan.world_center);
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
    mut q_cameras: Query<(&Camera, &mut OrthographicProjection, &GlobalTransform, &mut PancamViewport)>,
) {
    for (camera, projection, transform, mut viewport_pan) in &mut q_cameras {
        let viewport_size_old = camera.logical_viewport_size().unwrap();
        let viewport_size_new = camera.world_to_viewport(transform, (viewport_pan.world_size/2.0 + viewport_pan.world_center).extend(1.0));

        projection.scale


        viewport_pan.world_center = camera.viewport_to_world_2d(transform, viewport_size*0.5).unwrap();
        viewport_pan.world_size   = 2.0*(camera.viewport_to_world_2d(transform, viewport_size).unwrap() - viewport_pan.world_center);
    }
}