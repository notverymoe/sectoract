// Copyright 2023 Natalie Baker // AGPLv3 //

#![warn(
    clippy::all, 
    clippy::pedantic,
    clippy::alloc_instead_of_core,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::mixed_read_write_in_expression,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unwrap_in_result,
    clippy::missing_const_for_fn,
)]

#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::needless_pass_by_value, // Bevy systems need this
    clippy::shadow_unrelated,       // Egui lambda params
)]

use bevy::app::AppExit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use sectoract_editor::ui::GridCursor;
use sectoract_editor::{data, draw, ui};
use sectoract_level::map::Sector;

#[derive(Resource)]
pub struct SectorResource {
    pub sector: Sector,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<ui::ScreenLayout>()
        .init_resource::<ui::GridSettings>()
        .init_resource::<ui::GridCursor>()
        .insert_resource(SectorResource{
            sector: data::get_test_sector(),
        })
        .insert_resource(GizmoConfig{
            line_width: 0.5,
            ..Default::default()
        })
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, setup)
        .add_systems(PreUpdate,  (ui::update_physical_scale, input_grid_size, ui::update_grid_cursor, input_camera).chain())
        .add_systems(Update,     (ui_build, ui::update_camera_reserved_space).chain())
        .add_systems(PostUpdate, (draw_2d_grid, draw_2d_cursor, draw_2d_sector))
        .run();
}

// // Setup // //

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// // Input // //

fn input_grid_size(
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

fn input_camera(
    r_input_mouse: Res<Input<MouseButton>>,
    r_cursor: Res<GridCursor>,
    mut q_cameras: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
    time: Res<Time>,
) {

    if r_input_mouse.pressed(MouseButton::Middle) {
        let delta = r_cursor.delta_viewport_scaled;
        q_cameras.single_mut().1.translation -= delta.extend(0.0);
    }

    let mut scroll_delta = 0.0;
    for ev in scroll_evr.read() {
        scroll_delta += match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line  => -ev.y,
            bevy::input::mouse::MouseScrollUnit::Pixel => -ev.y / 14.0,
        };
    }

    if scroll_delta != 0.0 {
        let (mut projection, mut transform) = q_cameras.single_mut();
        zoom_camera(&mut projection, &mut transform, r_cursor.pos_world, scroll_delta * time.delta_seconds());
    }

}

fn zoom_camera(
    cam_projection: &mut OrthographicProjection, 
    cam_transform: &mut Transform, 
    mouse_world: Vec2,
    amount: f32
) {
    //let old_scale = cam_projection.scale;
    cam_projection.scale = (cam_projection.scale.ln() + amount).exp();

    //let offset_to_mouse = mouse_world.extend(cam_transform.translation.z) - cam_transform.translation;
    //let distance_to_mouse = offset_to_mouse.length();
    //let max_move_dist     = (cam_projection.area.size()/old_scale*cam_projection.scale).length() * 0.25;
    //let offset_scale      = distance_to_mouse.min(max_move_dist)/distance_to_mouse;

    //cam_transform.translation += offset_to_mouse*offset_scale;
}

// // Draw // //

fn draw_2d_grid(
    mut gizmos: Gizmos,
    r_grid_settings: Res<ui::GridSettings>,
    q_cameras: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    let (camera, camera_projection, camera_transform) = q_cameras.single();

    let size = r_grid_settings.size.get_units() as f32;
    let from = camera_transform.translation().truncate() - camera_projection.viewport_origin*camera_projection.area.size();
    let to   = from + camera.physical_viewport_size().unwrap_or(UVec2::ZERO).as_vec2();
    draw::grid_snapped(&mut gizmos, Vec2::ZERO, from, to, Vec2::new(size, size), Color::WHITE.with_a(0.1));
}

fn draw_2d_cursor(
    mut gizmos: Gizmos,
    r_cursor: Res<ui::GridCursor>,
) {

    if r_cursor.is_active {
        gizmos.circle_2d(r_cursor.pos_grid.as_vec2(), 4.0, Color::WHITE);
    }
}

fn draw_2d_sector(
    mut gizmos: Gizmos,
    sector_res: Res<SectorResource>,
) {
    draw::section_lines(&mut gizmos, &sector_res.sector, Color::RED, Color::GREEN);
}

// // UI // //

fn ui_build(
    mut contexts: EguiContexts,
    mut r_layout: ResMut<ui::ScreenLayout>,
    r_cursor: Res<ui::GridCursor>,
    mut r_grid_settings: ResMut<ui::GridSettings>,
    mut exit: EventWriter<AppExit>
) {
    let ctx = contexts.ctx_mut();

    r_layout.space.top = egui::TopBottomPanel::top("top").show(ctx, |ui| {
        ui.horizontal(|ui| {
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
            ui.menu_button("View", |ui| {
                if ui.button("Increase Grid Size").clicked() {
                    r_grid_settings.size.change(1);
                }
                if ui.button("Decrease Grid Size").clicked() {
                    r_grid_settings.size.change(-1);
                }
            });
        });
    }).response.rect.height() as u32;

    egui::Window::new("Tools")
        .resizable(false)
        .anchor(Align2::LEFT_TOP, egui::Vec2::new(4.0, 4.0))
        .show(ctx, |ui| {
            ui.label("Tools");
        });

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
                ui.label("Y: ---------");
            } 
        })
    }).response.rect.height() as u32;
}

fn format_coord(val: f32, scale: Option<f32>) -> String {
    // TODO pad sign
    let scale = scale.unwrap_or(val.abs())/40.0;
    let val   = val/40.0;
    if scale < 0.1 && (val.abs() < 10.0) {
        format!("{: >7.2}cm", val*100.0)
    } else {
        format!("{val: >7.2}m ")
    }
}