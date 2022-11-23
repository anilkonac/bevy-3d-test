use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod player;
mod ui;
use player::PlayerPlugin;
use ui::UIPlugin;

const COLOR_BACKGROUND: &str = "87CEEB"; // Sky Blue

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum AppState {
    Start,
    InGame,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::default())
            .insert_resource(ClearColor(Color::hex(COLOR_BACKGROUND).unwrap()))
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(PlayerPlugin)
            .add_plugin(UIPlugin)
            .add_state(AppState::Start)
            .add_startup_system(setup.label("main_setup"));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Dungeon
    commands.spawn(SceneBundle {
        scene: asset_server.load("dungeon.gltf#Scene0"),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::hex(COLOR_BACKGROUND).unwrap(),
        brightness: 0.9,
    });
}

// fn setup_lights(mut commands, Query<Transform>)
