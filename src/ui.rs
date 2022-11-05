use bevy::{
    pbr::{DirectionalLightShadowMap, PointLightShadowMap},
    {prelude::*, window::close_when_requested},
};
use bevy_egui::{
    egui::{self, Align2, Ui},
    EguiContext, EguiPlugin,
};

use crate::AppState;

const MENU_OFFSET: f32 = 10.0;

#[derive(PartialEq)]
enum LightType {
    Point,
    Directional,
}

// Resource
struct LightSettings {
    light_direct_illuminance: f32,
    light_point_intensity: f32,
    current_light: LightType,
}

impl Default for LightSettings {
    fn default() -> Self {
        Self {
            light_direct_illuminance: 100000.0,
            light_point_intensity: 800.0,
            current_light: LightType::Directional,
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::default())
            .insert_resource(LightSettings::default())
            .add_plugin(EguiPlugin)
            .add_system(ui_info.before(ui_graphics))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(ui_graphics.before(close_when_requested)),
            )
            .add_system(grab_mouse.label("grab_mouse").before(ui_info));
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
                window.set_cursor_lock_mode(false);
                app_state.set(AppState::Menu).unwrap();
            }
            AppState::Menu => {
                window.set_cursor_visibility(false);
                window.set_cursor_lock_mode(true);
                app_state.set(AppState::InGame).unwrap();
            }
            _ => (),
        }
    }

    if mouse.just_pressed(MouseButton::Left) && (*app_state.current() == AppState::Start) {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
        app_state.set(AppState::InGame).unwrap();
    }
}

fn ui_info(mut egui_context: ResMut<EguiContext>, app_state: Res<State<AppState>>) {
    let contents: fn(&mut Ui) = match app_state.current() {
        AppState::Start => |ui| {
            ui.label("Click on the game screen to start");
        },
        AppState::Menu => |ui| {
            ui.label("Press M to close the menu");
        },
        AppState::InGame => |ui| {
            ui.label("- Use the mouse to look");
            ui.label("- Use WASD or arrow keys to move");
            ui.label("- Press M for the menu");
        },
    };

    egui::Window::new("Info")
        .id(egui::Id::new("Info"))
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, [MENU_OFFSET, -MENU_OFFSET])
        .show(egui_context.ctx_mut(), contents);
    return;
}

fn ui_graphics(
    mut egui_context: ResMut<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut shadow_map_direct: ResMut<DirectionalLightShadowMap>,
    mut shadow_map_point: ResMut<PointLightShadowMap>,
    mut query_light_direct: Query<&mut DirectionalLight>,
    mut query_light_point: Query<&mut PointLight>,
    mut app_state: ResMut<State<AppState>>,
    mut windows: ResMut<Windows>,
    mut light_power: ResMut<LightSettings>,
) {
    egui::Window::new("Graphics")
        .anchor(Align2::LEFT_TOP, [MENU_OFFSET, MENU_OFFSET])
        .show(egui_context.ctx_mut(), |ui| {
            const STEP_SIZE_SHADOW_MAP: usize = 1024;

            let mut msaa_active = msaa.samples > 1;
            ui.checkbox(&mut msaa_active, "MSAA");
            if msaa_active {
                msaa.samples = 4;
            } else {
                msaa.samples = 1;
            }
            ui.separator();

            let mut light_point = query_light_point.single_mut();
            let mut light_direct = query_light_direct.single_mut();

            ui.horizontal(|ui| {
                ui.radio_value(&mut light_power.current_light, LightType::Point, "Point");
                ui.radio_value(
                    &mut light_power.current_light,
                    LightType::Directional,
                    "Directional",
                );
                ui.label("Light Type");
            });

            let show_shadow_projection: bool;
            let shadow_map_size = match light_power.current_light {
                LightType::Point => {
                    light_direct.illuminance = 0.0;
                    show_shadow_projection = false;
                    ui.add(
                        egui::Slider::new(&mut light_power.light_point_intensity, 0.0..=4000.0)
                            .text("Intensity"),
                    );
                    light_point.intensity = light_power.light_point_intensity;
                    &mut shadow_map_point.size
                }
                LightType::Directional => {
                    light_point.intensity = 0.0;
                    show_shadow_projection = true;
                    ui.add(
                        egui::Slider::new(
                            &mut light_power.light_direct_illuminance,
                            0.0..=100000.0,
                        )
                        .text("Illuminance"),
                    );
                    light_direct.illuminance = light_power.light_direct_illuminance;
                    &mut shadow_map_direct.size
                }
            };

            ui.separator();

            let mut shadow_projection_size = light_direct.shadow_projection.right;

            ui.add_enabled_ui(show_shadow_projection, |ui| {
                ui.add(
                    egui::Slider::new(&mut shadow_projection_size, 0.0..=200.0)
                        .text("Shadow Projection Size"),
                );
            });
            if show_shadow_projection {
                light_direct.shadow_projection = OrthographicProjection {
                    left: -shadow_projection_size,
                    right: shadow_projection_size,
                    bottom: -shadow_projection_size,
                    top: shadow_projection_size,
                    near: -shadow_projection_size,
                    far: shadow_projection_size,
                    ..default()
                };
            }

            ui.end_row();

            ui.add(
                egui::Slider::new(
                    shadow_map_size,
                    STEP_SIZE_SHADOW_MAP..=STEP_SIZE_SHADOW_MAP * 8,
                )
                .step_by(STEP_SIZE_SHADOW_MAP as f64)
                .text("Shadow Map Size"),
            );

            ui.separator();

            if ui.button("Close Menu").clicked() {
                let window = windows.get_primary_mut().unwrap();
                app_state.set(AppState::InGame).unwrap();
                window.set_cursor_visibility(false);
                window.set_cursor_lock_mode(true);
            }
        });
}
