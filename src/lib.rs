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

const SHADOW_PROJECTION_SIZE: f32 = HALF_SIZE_GROUND * 1.42/*~=2.0.sqrt()*/;

const TRANSLATION_LIGHT_POINT_SPOT: Vec3 = Vec3::new(-2.0, 2.5, 1.0);

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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: HALF_SIZE_GROUND * 2.0,
            })),
            material: materials.add(Color::hex(COLOR_GROUND).unwrap().into()),
            ..default()
        },
        Collider::cuboid(HALF_SIZE_GROUND, 0.0, HALF_SIZE_GROUND),
    ));

    // Create cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: HALF_SIZE_CUBE * 2.0,
            })),
            material: materials.add(Color::hex(COLOR_CUBE).unwrap().into()),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(HALF_SIZE_CUBE, HALF_SIZE_CUBE, HALF_SIZE_CUBE),
    ));

    // Create lights
    let transform_light_point = Transform::from_translation(TRANSLATION_LIGHT_POINT_SPOT);
    let transform_light_spot =
        Transform::from_translation(TRANSLATION_LIGHT_POINT_SPOT).looking_at(Vec3::ZERO, Vec3::Y);
    let transform_light_direct = Transform::from_rotation(transform_light_spot.rotation);

    commands.spawn(DirectionalLightBundle {
        transform: transform_light_direct,
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

    commands.spawn(PointLightBundle {
        transform: transform_light_point,
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 0.0,
            ..default()
        },
        ..default()
    });

    commands.spawn(SpotLightBundle {
        transform: transform_light_spot,
        spot_light: SpotLight {
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
