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
        app.add_plugin(EguiPlugin)
            .insert_resource(Msaa::default())
            .insert_resource(DirectionalLightShadowMap::default())
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(ui_graphics));
    }
}

fn ui_graphics(
    mut egui_context: ResMut<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut shadop_map: ResMut<DirectionalLightShadowMap>,
    mut query_light_direct: Query<&mut DirectionalLight>,
    // mut query_light_point: Query<&mut PointLight>,
) {
    egui::Window::new("Graphics").show(egui_context.ctx_mut(), |ui| {
        const STEP_SIZE_SHADOW_MAP: usize = 2048;

        let mut msaa_active = msaa.samples > 1;
        let mut type_light = LightType::Directional;
        ui.checkbox(&mut msaa_active, "MSAA");
        if msaa_active {
            msaa.samples = 4;
        } else {
            msaa.samples = 1;
        }
        ui.separator();

        ui.horizontal(|ui| {
            ui.radio_value(&mut type_light, LightType::Directional, "Directional");
            ui.radio_value(&mut type_light, LightType::Point, "Point");
            ui.label("Light Type");
        });

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

        let mut dir_light = query_light_direct.single_mut();
        ui.add(
            egui::Slider::new(&mut dir_light.shadow_depth_bias, -0.5..=1.5)
                .step_by(0.01)
                .text("Shadow Depth Bias"),
        );
        ui.end_row();
        ui.add(
            egui::Slider::new(&mut dir_light.shadow_normal_bias, -1.0..=10.0)
                .step_by(0.1)
                .text("Shadow Normal Bias"),
        );
    });
}
