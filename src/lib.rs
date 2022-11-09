use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod ui;
use player::PlayerPlugin;
use ui::UIPlugin;

const COLOR_BACKGROUND: &str = "87CEEB"; // Sky Blue
const COLOR_CUBE: &str = "FE4A49"; // Tart Orange
const COLOR_GROUND: &str = "586A6A"; // Deep Space Sparkle

const HALF_SIZE_GROUND: f32 = 7.5;
const HALF_SIZE_CUBE: f32 = 0.5;

const SHADOW_PROJECTION_SIZE: f32 = 20.0;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum AppState {
    Start,
    InGame,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex(COLOR_BACKGROUND).unwrap()))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(PlayerPlugin)
            .add_plugin(UIPlugin)
            .add_state(AppState::Start)
            .add_startup_system(setup.label("main_setup"));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create ground plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: HALF_SIZE_GROUND * 2.0,
            })),
            material: materials.add(Color::hex(COLOR_GROUND).unwrap().into()),
            ..default()
        })
        .insert(Collider::cuboid(HALF_SIZE_GROUND, 0.0, HALF_SIZE_GROUND));

    // Create cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: HALF_SIZE_CUBE * 2.0,
            })),
            material: materials.add(Color::hex(COLOR_CUBE).unwrap().into()),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            HALF_SIZE_CUBE,
            HALF_SIZE_CUBE,
            HALF_SIZE_CUBE,
        ));

    // Create lights
    let transform_lights = Transform::from_xyz(-3.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(DirectionalLightBundle {
        transform: transform_lights,
        directional_light: DirectionalLight {
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -SHADOW_PROJECTION_SIZE,
                right: SHADOW_PROJECTION_SIZE,
                bottom: -SHADOW_PROJECTION_SIZE,
                top: SHADOW_PROJECTION_SIZE,
                near: -SHADOW_PROJECTION_SIZE,
                far: SHADOW_PROJECTION_SIZE,
                ..default()
            },
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        transform: transform_lights,
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 0.0,
            ..default()
        },
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::hex(COLOR_BACKGROUND).unwrap(),
        brightness: 0.1,
    })
}
