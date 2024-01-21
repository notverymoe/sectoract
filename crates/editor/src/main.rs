// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use sectoract_editor::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<ui::ScreenLayout>()
        .init_resource::<ui::GridSettings>()
        .init_resource::<ui::GridCursor>()
        .insert_resource(GizmoConfig{
            line_width: 0.5,
            ..Default::default()
        })
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, (ui::update_physical_scale, control_grid, ui::update_grid_cursor).chain())
        .add_systems(Update, (ui_test_system, ui::update_camera_reserved_space, gizmo_grid).chain())
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

fn control_grid(
    keyboard: Res<Input<KeyCode>>,
    mut r_grid_settings: ResMut<ui::GridSettings>,
) {

    if keyboard.just_pressed(KeyCode::Equals) {
        r_grid_settings.size.change( 1);
    }

    if keyboard.just_pressed(KeyCode::Minus) {
        r_grid_settings.size.change(-1);
    }

}

fn gizmo_grid(
    mut gizmos: Gizmos,
    r_grid_settings: Res<ui::GridSettings>,
    r_cursor: Res<ui::GridCursor>,
    q_cameras: Query<&Camera>,
) {
    let size_grid = r_grid_settings.size.get_units();
    let size_view = q_cameras.single().logical_viewport_size().unwrap_or(Vec2::ZERO).as_uvec2();

    for x in (0..size_view.x).step_by(size_grid as usize) {
        gizmos.line_2d(Vec2::new(x as f32, 0.0), UVec2::new(x, size_view.y).as_vec2(), Color::WHITE.with_a(0.1));
    }

    for y in (0..size_view.y).step_by(size_grid as usize) {
        gizmos.line_2d(Vec2::new(0.0, y as f32), UVec2::new(size_view.x, y).as_vec2(), Color::WHITE.with_a(0.1));
    }

    if r_cursor.is_active {
        gizmos.circle_2d(r_cursor.pos_grid.as_vec2(), 4.0, Color::WHITE);
    }
}

fn ui_test_system(
    mut contexts: EguiContexts,
    mut r_layout: ResMut<ui::ScreenLayout>,
    r_cursor: Res<ui::GridCursor>,
    r_grid_settings: Res<ui::GridSettings>,
    mut exit: EventWriter<AppExit>
) {
    let ctx = contexts.ctx_mut();

    r_layout.space.top = egui::TopBottomPanel::top("top").show(ctx, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() {
                // TODO
                println!("New");
            }
            ui.separator();
            if ui.button("Open").clicked() {
                // TODO
                println!("Open");
            }
            ui.separator();
            if ui.button("Save").clicked() {
                // TODO
                println!("Save");
            }
            if ui.button("Save As...").clicked() {
                // TODO
                println!("Save As...");
            }
            ui.separator();
            if ui.button("Exit").clicked() {
                exit.send(AppExit);
            }
        });
    }).response.rect.height() as u32;

    ctx.style_mut(|s| s.override_text_style = Some(egui::TextStyle::Monospace));

    r_layout.space.bottom = egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
        let grid_size = r_grid_settings.size.get_units() as f32;
        ui.horizontal(|ui| {
            ui.label(format!(
                "Grid: {}", 
                format_coord(grid_size, None)
            ));
            ui.separator();
            if r_cursor.is_active {
                ui.label(format!("X: {}", format_coord(r_cursor.pos_grid.x as f32, Some(grid_size))));
                ui.separator();
                ui.label(format!("Y: {}", format_coord(r_cursor.pos_grid.y as f32, Some(grid_size))));
            } else {
                ui.label("X: ---------");
                ui.separator();
                ui.label("Y: --------- ");
            } 
        })
    }).response.rect.height() as u32;

    egui::Window::new("Tools")
        .resizable(false)
        .anchor(Align2::LEFT_TOP, egui::Vec2::new(4.0, 4.0))
        .show(ctx, |ui| {
            ui.label("Tools");
        });

}

fn format_coord(val: f32, scale: Option<f32>) -> String {
    // TODO pad sign
    let scale = scale.unwrap_or(val.abs())/40.0;
    let val   = val/40.0;
    if scale < 0.1 && (val.abs() < 10.0) {
        format!("{: >7.2}cm", val*100.0)
    } else {
        format!("{: >7.2}m ", val)
    }.to_owned()
}