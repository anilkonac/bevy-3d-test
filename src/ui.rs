use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::*,
    window::close_when_requested,
    window::{CursorGrabMode, PrimaryWindow},
};

use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
// use bevy_inspector_egui::{widgets::InspectorQuery, InspectorPlugin, WorldInspectorPlugin};

use crate::{
    player::{CAMERA_TPS_POS_RELATIVE, HEAD_SIZE},
    AppState, PointLightSettings,
};

#[cfg(not(target_arch = "wasm32"))]
const IS_DESKTOP_BUILD: bool = true;
#[cfg(target_arch = "wasm32")]
static IS_DESKTOP_BUILD: bool = false;

#[derive(PartialEq, Clone, Copy)]
enum CameraType {
    FirstPerson,
    ThirdPerson,
}

#[derive(Resource)]
pub struct CameraSettings {
    c_type: CameraType,
    distance: f32,
    pub bloom: BloomSettings,
    pub bloom_enabled: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            c_type: CameraType::ThirdPerson,
            distance: CAMERA_TPS_POS_RELATIVE.distance(Vec3::ZERO),
            // bloom: BloomSettings {
            //     intensity: 0.002,
            //     scale: 1.40,
            //     ..default()
            // },
            bloom: BloomSettings::default(),
            bloom_enabled: IS_DESKTOP_BUILD,
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            // .add_plugin(InspectorPlugin::<InspectorQuery<&mut PointLight>>::new())
            // .add_plugin(WorldInspectorPlugin::default())
            .insert_resource(CameraSettings::default())
            .add_system(ui_info.before(ui_graphics))
            // .add_system(ui_graphics.in_set(OnUpdate(AppState::Menu).before(ui_camera)))
            // .add_system(ui_camera.in_set(OnUpdate(AppState::Menu).before(close_when_requested)))
            .add_system(grab_mouse_system.before(ui_info))
            .add_system(switch_camera.before(ui_camera));
    }
}

pub fn grab_mouse_system(
    mut windows: Query<&mut Window>,
    app_state: Res<State<AppState>>,
    mut app_state_queue: ResMut<NextState<AppState>>,
    key: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    if key.just_pressed(KeyCode::M) {
        let mut window = windows.single_mut();

        match app_state.0 {
            AppState::InGame => {
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
            AppState::Menu => {
                window.cursor.visible = false;
                app_state_queue.set(AppState::InGame);
                grab_mouse(&mut window);
            }
            _ => (),
        }
    }

    if mouse.just_pressed(MouseButton::Left) && (app_state.0 == AppState::Start) {
        let mut window = windows.single_mut();
        window.cursor.visible = false;
        app_state_queue.set(AppState::InGame);
        grab_mouse(&mut window);
    }
}

#[cfg(any(target_os = "macos", target_arch = "wasm32"))]
fn grab_mouse(window: &mut Window) {
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

#[cfg(not(any(target_os = "macos", target_arch = "wasm32")))]
fn grab_mouse(window: &mut Window) {
    window.cursor.grab_mode = CursorGrabMode::Confined;
}

fn ui_info(mut egui_contexts: EguiContexts, app_state: Res<State<AppState>>) {
    let contents: fn(&mut Ui) = match app_state.0 {
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
        .show(egui_contexts.ctx_mut(), contents);
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
    mut egui_contexts: EguiContexts,
    mut query_light_point: Query<&mut PointLight>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient_light: ResMut<AmbientLight>,
    mut plight_settings: ResMut<PointLightSettings>,
) {
    let contents = |ui: &mut Ui| {
        let mut color_lrgba_clear = clear_color.as_linear_rgba_f32();
        let mut color_lrgba_ambient = ambient_light.color.as_linear_rgba_f32();
        let mut color_lrgba_point = plight_settings.light.color.as_linear_rgba_f32();

        ui.horizontal(|ui| {
            ui.label("Clear Color");
            ui.color_edit_button_rgba_unmultiplied(&mut color_lrgba_clear);
            ui.label("Sync with");
            if ui.button("Ambient").clicked() {
                color_lrgba_ambient = color_lrgba_clear;
            }
        });
        clear_color.0 = Color::rgba_linear(
            color_lrgba_clear[0],
            color_lrgba_clear[1],
            color_lrgba_clear[2],
            color_lrgba_clear[3],
        );

        ui.separator();

        ui.label("Ambient Light ");
        ui.horizontal(|ui| {
            ui.label("Color");
            ui.color_edit_button_rgba_unmultiplied(&mut color_lrgba_ambient);
            ui.label("Brightness");
            ui.add(egui::Slider::new(&mut ambient_light.brightness, 0.0..=5.0).step_by(0.01));
        });
        ambient_light.color = Color::rgba_linear(
            color_lrgba_ambient[0],
            color_lrgba_ambient[1],
            color_lrgba_ambient[2],
            color_lrgba_ambient[3],
        );

        ui.separator();
        ui.label("Point Lights");
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Color");
            changed |= ui
                .color_edit_button_rgba_unmultiplied(&mut color_lrgba_point)
                .changed();
            ui.label("Intensity");
            changed |= ui
                .add(egui::Slider::new(
                    &mut plight_settings.light.intensity,
                    0.0..=4000.0,
                ))
                .changed();
        });
        changed |= ui
            .checkbox(&mut plight_settings.light.shadows_enabled, "Shadows")
            .changed();
        if !IS_DESKTOP_BUILD {
            ui.label(
                "! Currently, shadows can only be enabled for one point light in WebAssembly !",
            );
        }
        if changed {
            plight_settings.light.color = Color::rgba_linear(
                color_lrgba_point[0],
                color_lrgba_point[1],
                color_lrgba_point[2],
                color_lrgba_point[3],
            );
            for mut point_light in query_light_point.iter_mut() {
                point_light.color = plight_settings.light.color;
                point_light.intensity = plight_settings.light.intensity;
                point_light.shadows_enabled = plight_settings.light.shadows_enabled;
            }
        }
    };

    egui::Window::new("Graphics").show(egui_contexts.ctx_mut(), contents);
}

fn ui_camera(
    mut egui_contexts: EguiContexts,
    mut cam_settings: ResMut<CameraSettings>,
    mut query_cams: Query<(&mut Camera, &mut Transform), With<Camera>>,
    mut query_bloom: Query<&mut BloomSettings>,
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
        ui.separator();

        // ui.add_enabled_ui(IS_DESKTOP_BUILD, |ui| {
        //     let mut changed = false;
        //     changed |= ui
        //         .checkbox(&mut cam_settings.bloom_enabled, "Bloom")
        //         .changed();
        //     ui.horizontal(|ui| {
        //         ui.label("Threshold");
        //         changed |= ui
        //             .add(egui::DragValue::new(&mut cam_settings.bloom.threshold).speed(0.01))
        //             .changed();
        //     });
        //     ui.horizontal(|ui| {
        //         ui.label("Knee");
        //         changed |= ui
        //             .add(egui::DragValue::new(&mut cam_settings.bloom.knee).speed(0.01))
        //             .changed();
        //     });
        //     ui.horizontal(|ui| {
        //         ui.label("Scale");
        //         changed |= ui
        //             .add(egui::DragValue::new(&mut cam_settings.bloom.scale).speed(0.01))
        //             .changed();
        //     });
        //     ui.horizontal(|ui| {
        //         ui.label("Intensity");
        //         changed |= ui
        //             .add(egui::DragValue::new(&mut cam_settings.bloom.intensity).speed(0.0001))
        //             .changed();
        //     });

        //     if changed {
        //         for mut bloom in query_bloom.iter_mut() {
        //             *bloom = cam_settings.bloom.clone();
        //         }
        //         for (mut cam, _) in query_cams.iter_mut() {
        //             cam.hdr = cam_settings.bloom_enabled;
        //         }
        //     }
        // });
        // if !IS_DESKTOP_BUILD {
        //     ui.label("! Currently, bloom effect does not work in WebAssembly !");
        // }
    };

    egui::Window::new("Camera")
        .id(egui::Id::new("Camera"))
        .show(egui_contexts.ctx_mut(), contents);
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
