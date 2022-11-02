use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
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
            .add_plugin(EguiPlugin)
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(ui_graphics));
    }
}

fn ui_graphics(
    mut egui_context: ResMut<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut shadop_map: ResMut<DirectionalLightShadowMap>,
    mut query_light_direct: Query<&mut DirectionalLight>,
    mut query_light_point: Query<&mut PointLight>,
) {
    egui::Window::new("Graphics").show(egui_context.ctx_mut(), |ui| {
        const STEP_SIZE_SHADOW_MAP: usize = 2048;

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

        match light_type {
            LightType::Point => {
                light_direct.illuminance = 0.0;
                light_point.intensity = 800.0;
            }
            LightType::Directional => {
                light_direct.illuminance = 100000.0;
                light_point.intensity = 0.0;
            }
        }

        ui.separator();

        ui.add(
            egui::Slider::new(
                &mut shadop_map.size,
                STEP_SIZE_SHADOW_MAP..=STEP_SIZE_SHADOW_MAP * 8,
            )
            .step_by(STEP_SIZE_SHADOW_MAP as f64)
            .text("Shadow Map Size"),
        );
        ui.end_row();

        let mut shadow_depth_bias = light_point.shadow_depth_bias;
        let mut shadow_normal_bias = light_point.shadow_normal_bias;
        ui.add(
            egui::Slider::new(&mut shadow_depth_bias, -0.5..=1.5)
                .step_by(0.01)
                .text("Shadow Depth Bias"),
        );
        ui.end_row();
        ui.add(
            egui::Slider::new(&mut shadow_normal_bias, -1.0..=10.0)
                .step_by(0.1)
                .text("Shadow Normal Bias"),
        );

        light_point.shadow_depth_bias = shadow_depth_bias;
        light_point.shadow_normal_bias = shadow_normal_bias;
        light_direct.shadow_depth_bias = shadow_depth_bias;
        light_direct.shadow_normal_bias = shadow_normal_bias;
    });
}
