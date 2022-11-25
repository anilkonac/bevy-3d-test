use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod ui;
use player::PlayerPlugin;
use ui::UIPlugin;

const COLOR_BACKGROUND: Color = Color::rgb_linear(0.008, 0.008, 0.011);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum AppState {
    Start,
    InGame,
    Menu,
}

#[derive(Resource, Default)]
pub struct PointLightSettings {
    pub intensity: f32,
    pub color: Color,
    pub initialized: bool,
    pub shadows_enabled: bool,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::default())
            .insert_resource(ClearColor(COLOR_BACKGROUND))
            .insert_resource(PointLightSettings::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(PlayerPlugin)
            .add_plugin(UIPlugin)
            .add_state(AppState::Start)
            .add_startup_system(setup.label("main_setup"))
            .add_system_set(SystemSet::on_update(AppState::Start).with_system(setup_lights));
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
            settings.intensity = light.intensity;

            // Treat the color coming from Blender as rgba_linear
            let blender_color = light.color.as_rgba_f32();
            settings.color = Color::rgba_linear(
                blender_color[0],
                blender_color[1],
                blender_color[2],
                blender_color[3],
            );

            settings.initialized = true;
            settings.shadows_enabled = true;
            ambient_light.color = settings.color;
        }
        light.shadows_enabled = true;
        light.color = settings.color;
    }
}
