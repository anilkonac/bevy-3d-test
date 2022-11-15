use bevy::{
    pbr::{DirectionalLightShadowMap, PointLightShadowMap},
    {prelude::*, window::close_when_requested, window::CursorGrabMode},
};
use bevy_egui::{
    egui::{self, Ui},
    EguiContext, EguiPlugin,
};

use crate::{
    player::{CAMERA_TPS_POS_RELATIVE, HEAD_SIZE},
    AppState,
};

#[derive(PartialEq)]
enum LightType {
    Directional,
    Point,
    Spot,
}

#[derive(PartialEq, Clone, Copy)]
enum CameraType {
    FirstPerson,
    ThirdPerson,
}

#[derive(Resource)]
struct LightSettings {
    light_direct_illuminance: f32,
    light_point_intensity: f32,
    light_spot_intensity: f32,
    current_type: LightType,
}

impl Default for LightSettings {
    fn default() -> Self {
        LightSettings {
            light_direct_illuminance: 100000.0,
            light_point_intensity: 800.0,
            light_spot_intensity: 800.0,
            current_type: LightType::Directional,
        }
    }
}

#[derive(Resource)]
struct CameraSettings {
    c_type: CameraType,
    distance: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            c_type: CameraType::ThirdPerson,
            distance: CAMERA_TPS_POS_RELATIVE.distance(Vec3::ZERO),
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LightSettings::default())
            .insert_resource(CameraSettings::default())
            .add_plugin(EguiPlugin)
            .add_system(ui_info.before(ui_graphics))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(ui_graphics.before(ui_camera))
                    .with_system(ui_camera.before(close_when_requested)),
            )
            .add_system(grab_mouse.label("grab_mouse").before(ui_info))
            .add_system(switch_camera.before(ui_camera));
    }
}

fn grab_mouse(
    mut windows: ResMut<Windows>,
    mut app_state: ResMut<State<AppState>>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    if key.just_pressed(KeyCode::M) {
        let window = windows.get_primary_mut().unwrap();

        match app_state.current() {
            AppState::InGame => {
                window.set_cursor_visibility(true);
                window.set_cursor_grab_mode(CursorGrabMode::None);
                app_state.set(AppState::Menu).unwrap();
            }
            AppState::Menu => {
                window.set_cursor_visibility(false);
                app_state.set(AppState::InGame).unwrap();
                if cfg!(target_os = "macos") {
                    window.set_cursor_grab_mode(CursorGrabMode::Locked);
                    return;
                }
                window.set_cursor_grab_mode(CursorGrabMode::Confined);
            }
            _ => (),
        }
    }

    if mouse.just_pressed(MouseButton::Left) && (*app_state.current() == AppState::Start) {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_visibility(false);
        app_state.set(AppState::InGame).unwrap();
        if cfg!(target_os = "macos") {
            window.set_cursor_grab_mode(CursorGrabMode::Locked);
            return;
        }
        window.set_cursor_grab_mode(CursorGrabMode::Confined);
    }
}

fn ui_info(mut egui_context: ResMut<EguiContext>, app_state: Res<State<AppState>>) {
    let contents: fn(&mut Ui) = match app_state.current() {
        AppState::Start => |ui| {
            ui.label("Click on the game screen to start");
        },
        AppState::Menu => |ui| {
            ui.label("Press M to close all menus");
        },
        AppState::InGame => |ui| {
            ui.label("- Use the mouse to look");
            ui.label("- Use WASD or arrow keys to move");
            ui.label("- Press C to switch camera");
            ui.label("- Press M for the settings menu");
        },
    };

    egui::Window::new("Info")
        .id(egui::Id::new("Info"))
        .collapsible(false)
        .resizable(false)
        .show(egui_context.ctx_mut(), contents);
}

fn switch_camera(
    key: Res<Input<KeyCode>>,
    mut cam_settings: ResMut<CameraSettings>,
    mut query_cams: Query<&mut Camera>,
) {
    if key.just_pressed(KeyCode::C) {
        cam_settings.c_type = match cam_settings.c_type {
            CameraType::FirstPerson => CameraType::ThirdPerson,
            CameraType::ThirdPerson => CameraType::FirstPerson,
        };

        for mut cam in query_cams.iter_mut() {
            cam.is_active = !cam.is_active;
        }
    }
}

fn ui_graphics(
    mut egui_context: ResMut<EguiContext>,
    mut shadow_map_direct: ResMut<DirectionalLightShadowMap>,
    mut shadow_map_point: ResMut<PointLightShadowMap>,
    mut query_light_direct: Query<&mut DirectionalLight>,
    mut query_light_point: Query<&mut PointLight>,
    mut query_light_spot: Query<&mut SpotLight>,
    mut light_settings: ResMut<LightSettings>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    const STEP_SIZE_SHADOW_MAP: usize = 1024;

    let contents = |ui: &mut Ui| {
        let mut light_direct = query_light_direct.single_mut();
        let mut light_point = query_light_point.single_mut();
        let mut light_spot = query_light_spot.single_mut();

        let mut color_rgba_clear = clear_color.as_rgba_f32();
        let mut color_rgba_ambient = ambient_light.color.as_rgba_f32();

        ui.horizontal(|ui| {
            ui.label("Clear Color");
            ui.color_edit_button_rgba_unmultiplied(&mut color_rgba_clear);
            ui.label("Sync with");
            if ui.button("Ambient").clicked() {
                color_rgba_ambient = color_rgba_clear;
            }
            if ui.button("Light").clicked() {
                light_point.color = Color::from(color_rgba_clear);
                light_direct.color = Color::from(color_rgba_clear);
                light_spot.color = Color::from(color_rgba_clear);
            }
        });
        clear_color.0 = Color::from(color_rgba_clear);

        ui.separator();

        ui.label("Ambient Light ");
        ui.horizontal(|ui| {
            ui.label("Color");
            ui.color_edit_button_rgba_unmultiplied(&mut color_rgba_ambient);
            ui.label("Brightness");
            ui.add(egui::Slider::new(&mut ambient_light.brightness, 0.0..=1.0).step_by(0.01));
        });
        ambient_light.color = Color::from(color_rgba_ambient);

        ui.separator();

        ui.end_row();

        ui.horizontal(|ui| {
            ui.radio_value(
                &mut light_settings.current_type,
                LightType::Directional,
                "Directional",
            );
            ui.radio_value(&mut light_settings.current_type, LightType::Point, "Point");
            ui.radio_value(&mut light_settings.current_type, LightType::Spot, "Spot");

            ui.label("Light Type");
        });

        ui.end_row();

        // TODO: Reduce duplicate code
        let shadow_map_size = match light_settings.current_type {
            LightType::Directional => {
                // Disable other lights
                light_point.intensity = 0.0;
                light_spot.intensity = 0.0;

                let mut color = light_direct.color.as_rgba_f32();
                ui.horizontal(|ui| {
                    ui.label("Color");
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });
                light_direct.color = Color::from(color);
                ui.add(
                    egui::Slider::new(&mut light_settings.light_direct_illuminance, 0.0..=100000.0)
                        .text("Illuminance"),
                );
                light_direct.illuminance = light_settings.light_direct_illuminance;
                &mut shadow_map_direct.size
            }
            LightType::Point => {
                // Disable other lights
                light_direct.illuminance = 0.0;
                light_spot.intensity = 0.0;

                let mut color = light_point.color.as_rgba_f32();
                ui.horizontal(|ui| {
                    ui.label("Color");
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });

                light_point.color = Color::from(color);
                ui.add(
                    egui::Slider::new(&mut light_settings.light_point_intensity, 0.0..=4000.0)
                        .text("Intensity"),
                );
                light_point.intensity = light_settings.light_point_intensity;
                &mut shadow_map_point.size
            }
            LightType::Spot => {
                // Disable other lights
                light_direct.illuminance = 0.0;
                light_point.intensity = 0.0;

                let mut color = light_spot.color.as_rgba_f32();
                ui.horizontal(|ui| {
                    ui.label("Color");
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                });
                light_spot.color = Color::from(color);
                ui.add(
                    egui::Slider::new(&mut light_settings.light_spot_intensity, 0.0..=4000.0)
                        .text("Intensity"),
                );
                light_spot.intensity = light_settings.light_spot_intensity;
                &mut shadow_map_point.size
            }
        };

        ui.separator();

        ui.add_enabled(
            light_settings.current_type != LightType::Spot,
            egui::Slider::new(
                shadow_map_size,
                STEP_SIZE_SHADOW_MAP..=STEP_SIZE_SHADOW_MAP * 8,
            )
            .step_by(STEP_SIZE_SHADOW_MAP as f64)
            .text("Shadow Map Size"),
        );
    };

    egui::Window::new("Graphics").show(egui_context.ctx_mut(), contents);
}

fn ui_camera(
    mut egui_context: ResMut<EguiContext>,
    mut cam_settings: ResMut<CameraSettings>,
    mut query_cams: Query<(&mut Camera, &mut Transform), With<Camera>>,
) {
    let contents = |ui: &mut Ui| {
        ui.horizontal(|ui| {
            let cam_settings_prev = cam_settings.c_type;
            ui.radio_value(
                &mut cam_settings.c_type,
                CameraType::FirstPerson,
                "First Person",
            );
            ui.radio_value(
                &mut cam_settings.c_type,
                CameraType::ThirdPerson,
                "Third Person",
            );

            if cam_settings_prev != cam_settings.c_type {
                for (mut cam, _) in query_cams.iter_mut() {
                    cam.is_active = !cam.is_active;
                }
            }
        });

        ui.separator();

        for (cam, mut transform) in query_cams.iter_mut() {
            if !cam.is_active {
                continue;
            }

            ui.horizontal(|ui| match cam_settings.c_type {
                CameraType::ThirdPerson => {
                    ui.label("Distance");
                    let translation = &transform.translation;
                    let distance = &mut cam_settings.distance;
                    if ui
                        .add(egui::DragValue::new(distance).clamp_range(1.0..=200.0))
                        .changed()
                    {
                        let new_transform: Transform;
                        if (translation.x == 0.0) && (translation.z == 0.0) {
                            new_transform = Transform::from_translation(Vec3::Y * *distance)
                                .looking_at(Vec3::ZERO, -Vec3::Z);
                        } else if translation.x == 0.0 {
                            new_transform = compute_new_transform_without_x(translation, *distance);
                        } else {
                            new_transform = compute_new_transform(translation, *distance);
                        }
                        *transform = new_transform;
                    }
                }
                CameraType::FirstPerson => {
                    ui.label("Distance");
                    ui.add(
                        egui::Slider::new(&mut transform.translation.z, -HEAD_SIZE..=HEAD_SIZE)
                            .step_by(0.05),
                    );
                }
            });

            if cam_settings.c_type == CameraType::ThirdPerson {
                ui.separator();
                let translation = &mut transform.translation;

                ui.label("Translation");
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("X");
                        changed |= ui.add(egui::DragValue::new(&mut translation.x)).changed();
                    });
                    ui.horizontal(|ui| {
                        ui.label("Y");
                        changed |= ui.add(egui::DragValue::new(&mut translation.y)).changed();
                    });
                    ui.horizontal(|ui| {
                        ui.label("Z");
                        changed |= ui.add(egui::DragValue::new(&mut translation.z)).changed();
                    })
                });

                if !changed {
                    continue;
                }

                cam_settings.distance = translation.distance(Vec3::ZERO);
                if translation.x == 0.0 && translation.z == 0.0 {
                    *transform =
                        Transform::from_translation(*translation).looking_at(Vec3::ZERO, -Vec3::Z);
                } else {
                    *transform =
                        Transform::from_translation(*translation).looking_at(Vec3::ZERO, Vec3::Y);
                }
            }
        }
    };

    egui::Window::new("Camera")
        .id(egui::Id::new("Camera"))
        .show(egui_context.ctx_mut(), contents);
}

#[inline]
fn compute_new_transform_without_x(old_trans: &Vec3, new_distance: f32) -> Transform {
    let tan_beta = old_trans.y / old_trans.z;
    let tan_beta_sq = tan_beta * tan_beta;
    let z_sq = new_distance * new_distance / (1.0 + tan_beta_sq);
    let z = z_sq.sqrt() * old_trans.z.signum();
    let y = z * tan_beta;

    Transform::from_xyz(0.0, y, z).looking_at(Vec3::ZERO, Vec3::Y)
}

#[inline]
fn compute_new_transform(old_trans: &Vec3, new_distance: f32) -> Transform {
    // Of course, there has to be a better approach.
    let tan_alpha = old_trans.z / old_trans.x;
    let tan_alpha_sq = tan_alpha * tan_alpha;
    let d1_sq = old_trans.x * old_trans.x + old_trans.z * old_trans.z;
    let tan_beta_sq = old_trans.y * old_trans.y / d1_sq;

    let x_sq = new_distance * new_distance / ((1.0 + tan_alpha_sq) * (1.0 + tan_beta_sq));
    let x = x_sq.sqrt() * old_trans.x.signum();
    let z_sq = x_sq * tan_alpha_sq;
    let z = x * tan_alpha;
    let y_sq = (x_sq + z_sq) * tan_beta_sq;
    let y = y_sq.sqrt() * old_trans.y.signum();

    Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Y)
}
