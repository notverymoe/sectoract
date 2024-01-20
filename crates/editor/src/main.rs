// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use camera::update_camera_reserved_space;

mod camera;
use camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<CameraReservedSpace>()
        .insert_resource(GridSettings{
            size: 4,
        })
        .insert_resource(GizmoConfig{
            line_width: 0.5,
            ..Default::default()
        })
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, setup)
        .add_systems(Update, (control_grid, ui_test_system, update_camera_reserved_space, gizmo_grid).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle{
        projection: OrthographicProjection{
            viewport_origin: Vec2::ZERO,
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Resource)]
pub struct GridSettings {
    pub size: u32,
}

impl GridSettings {

    pub fn change_grid_size(&mut self, delta: i32) {
        if delta < 0 {
            self.size = self.size.saturating_sub(delta.unsigned_abs())
        } else {
            self.size = self.size.saturating_add(delta as u32).min(11);
        }
    }

    pub fn get_grid_size_units(&self) -> usize {
        match self.size {
             0 =>   1, // 2.5cm
             1 =>   2, // 5.0cm

             2 =>   4, // 10.0cm
             3 =>  10, // 25.0cm
             4 =>  20, // 50.0cm

             5 =>  40, // 1.0m
             6 => 100, // 2.5m
             7 => 200, // 5.0m
            
             8 =>  400, // 10m
             9 => 1000, // 25m
            10 => 2000, // 50m

            _ => 4000, // 100m
        }
    }

}

fn control_grid(
    keyboard: Res<Input<KeyCode>>,
    mut r_grid_settings: ResMut<GridSettings>,
) {

    if keyboard.just_pressed(KeyCode::Equals) {
        r_grid_settings.change_grid_size( 1);
    }

    if keyboard.just_pressed(KeyCode::Minus) {
        r_grid_settings.change_grid_size(-1);
    }

}

fn gizmo_grid(
    mut gizmos: Gizmos,
    r_grid_settings: Res<GridSettings>,
    q_cameras: Query<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let size_grid = r_grid_settings.get_grid_size_units();

    let window = q_windows.single();
    let (camera, camera_transform) = q_cameras.single();
    let cursor_world = get_cursor_world(window, camera, camera_transform);
    let cursor_grid  = (cursor_world / (size_grid as f32)).round() * (size_grid as f32);

    let size_view = camera.logical_viewport_size().unwrap_or(Vec2::ZERO).as_uvec2();


    for x in (0..size_view.x).step_by(size_grid) {
        gizmos.line_2d(Vec2::new(x as f32, 0.0), UVec2::new(x, size_view.y).as_vec2(), Color::WHITE.with_a(0.1));
    }

    for y in (0..size_view.y).step_by(size_grid) {
        gizmos.line_2d(Vec2::new(0.0, y as f32), UVec2::new(size_view.x, y).as_vec2(), Color::WHITE.with_a(0.1));
    }

    gizmos.circle_2d(cursor_grid, 4.0, Color::WHITE);
}

fn get_cursor_world(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let cursor_pos_viewport = window.cursor_position().unwrap_or(Vec2::ZERO) - camera.viewport.as_ref().map(|v| v.physical_position.as_vec2()).unwrap_or(Vec2::ZERO);
    camera.viewport_to_world_2d(camera_transform, cursor_pos_viewport).unwrap_or(Vec2::ZERO)
}

fn ui_test_system(
    mut contexts: EguiContexts,
    mut r_space: ResMut<CameraReservedSpace>,
    r_grid_settings: Res<GridSettings>,
    q_cameras: Query<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let ctx = contexts.ctx_mut();

    let size_grid = r_grid_settings.get_grid_size_units() as f32;

    let window = q_windows.single();
    let (camera, camera_transform) = q_cameras.single();
    let cursor_world = get_cursor_world(window, camera, camera_transform);
    let cursor_grid  = (cursor_world / size_grid).round() * size_grid;

    r_space.top = egui::TopBottomPanel::top("top").show(ctx, |ui| {
        ui.label("Top Panel");
    }).response.rect.height();

    r_space.bottom = egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let size_grid_metres = size_grid/40.0;
            if size_grid_metres < 0.1 {
                ui.label(format!("Grid: {}cm", size_grid_metres*100.0));
            } else {
                ui.label(format!("Grid: {}m", size_grid_metres));
            }

            ui.separator();

            ui.label(format!("X: {} | Y: {}", cursor_grid.x, cursor_grid.y));
        })
    }).response.rect.height();

    egui::Window::new("Tools")
        .resizable(false)
        .anchor(Align2::LEFT_TOP, egui::Vec2::new(4.0, 4.0))
        .show(ctx, |ui| {
            ui.label("Tools");
        });

}