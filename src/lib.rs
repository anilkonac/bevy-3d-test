use std::default;

use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;

mod player;
mod ui;
use player::PlayerPlugin;
use ui::UIPlugin;

const COLOR_BACKGROUND: Color = Color::rgb_linear(0.008, 0.008, 0.011);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Start,
    InGame,
    Menu,
}

#[derive(Resource)]
pub struct PointLightSettings {
    light: PointLight,
    initialized: bool,
}

impl Default for PointLightSettings {
    fn default() -> Self {
        PointLightSettings {
            light: PointLight {
                intensity: 700.0,
                ..default()
            },
            initialized: false,
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::default())
            .insert_resource(ClearColor(COLOR_BACKGROUND))
            .insert_resource(PointLightSettings::default())
            // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(PlayerPlugin)
            .add_plugin(UIPlugin)
            .add_state()
            .add_startup_system(setup.label("main_setup"))
            .add_system(setup_lights.in_set(OnUpdate(AppState::Start)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Dungeon
    commands.spawn(SceneBundle {
        scene: asset_server.load("dungeon.gltf#Scene0"),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 0.1,
        ..default()
    });
}

fn setup_lights(
    mut point_lights: Query<&mut PointLight>,
    mut settings: ResMut<PointLightSettings>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    if settings.initialized {
        return;
    }

    for mut light in point_lights.iter_mut() {
        if !settings.initialized {
            // Treat the color coming from Blender as rgba_linear
            let blender_color = light.color.as_rgba_f32();
            settings.light.color = Color::rgba_linear(
                blender_color[0],
                blender_color[1],
                blender_color[2],
                blender_color[3],
            );

            settings.initialized = true;
            settings.light.shadows_enabled = true;
            ambient_light.color = settings.light.color;
        }
        light.shadows_enabled = true;
        light.color = settings.light.color;
        light.intensity = settings.light.intensity;
    }
}
