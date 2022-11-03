use bevy::{
    pbr::{DirectionalLightShadowMap, PointLightShadowMap},
    {prelude::*, window::close_when_requested},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::AppState;

#[derive(PartialEq)]
enum LightType {
    Point,
    Directional,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::default())
            .insert_resource(DirectionalLightShadowMap::default())
            .insert_resource(PointLightShadowMap::default())
            .add_plugin(EguiPlugin)
            .add_system(ui_graphics.before("grab_mouse"))
            .add_system(grab_mouse.label("grab_mouse").before(close_when_requested));
    }
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
) {
    if *app_state.current() == AppState::InGame {
        egui::Window::new("Press M for the menu")
            .id(egui::Id::new("graphics"))
            .collapsible(false)
            .resizable(false)
            .show(egui_context.ctx_mut(), |_| {});
        return;
    }

    egui::Window::new("Graphics")
        .id(egui::Id::new("graphics"))
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

            let mut light_type = match light_point.intensity {
                x if x > 0.0 => LightType::Point,
                _ => LightType::Directional,
            };

            ui.horizontal(|ui| {
                ui.radio_value(&mut light_type, LightType::Point, "Point");
                ui.radio_value(&mut light_type, LightType::Directional, "Directional");
                ui.label("Light Type");
            });

            let shadow_map_size = match light_type {
                LightType::Point => {
                    light_direct.illuminance = 0.0;
                    light_point.intensity = 800.0;
                    &mut shadow_map_point.size
                }
                LightType::Directional => {
                    light_direct.illuminance = 100000.0;
                    light_point.intensity = 0.0;
                    &mut shadow_map_direct.size
                }
            };

            ui.separator();

            ui.add(
                egui::Slider::new(
                    shadow_map_size,
                    STEP_SIZE_SHADOW_MAP..=STEP_SIZE_SHADOW_MAP * 10,
                )
                .step_by(STEP_SIZE_SHADOW_MAP as f64)
                .text("Shadow Map Size"),
            );
            ui.end_row();

            ui.add(
                egui::Slider::new(&mut light_point.shadow_depth_bias, -0.5..=1.5)
                    .step_by(0.01)
                    .text("Shadow Depth Bias"),
            );
            ui.end_row();
            ui.add(
                egui::Slider::new(&mut light_point.shadow_normal_bias, -1.0..=10.0)
                    .step_by(0.1)
                    .text("Shadow Normal Bias"),
            );

            light_direct.shadow_depth_bias = light_point.shadow_depth_bias;
            light_direct.shadow_normal_bias = light_point.shadow_normal_bias;

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Close Menu").clicked() {
                    let window = windows.get_primary_mut().unwrap();
                    app_state.set(AppState::InGame).unwrap();
                    window.set_cursor_visibility(false);
                    window.set_cursor_lock_mode(true);
                }
                ui.label("or press M");
            })
        });
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
        }
    }

    if mouse.just_pressed(MouseButton::Left) && (*app_state.current() == AppState::InGame) {
        let window = windows.get_primary_mut().unwrap();
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
    }
}
