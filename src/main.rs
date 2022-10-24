use bevy::{prelude::*, time::FixedTimestep};

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system),
        )
        .add_system(player_movement_system)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::GRAY.into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::TEAL.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 80000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Camera
    let camera_height = 1.5;
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-5.0, camera_height, -4.0).looking_at(
                Vec3 {
                    x: 0.0,
                    y: camera_height,
                    z: 0.0,
                },
                Vec3::Y,
            ),
            ..default()
        })
        .insert(Player);
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    let mut movement_factor = 0.0;
    if keyboard_input.pressed(KeyCode::W) {
        movement_factor = -1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        info!("A is pressed");
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement_factor = 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        info!("D is pressed");
    }

    // get player's forward vector
    let movement_direction = transform.rotation * Vec3::Z;

    let movement_distance = movement_factor * PLAYER_SPEED * TIME_STEP;
    let translation_delta = movement_direction * movement_distance;

    transform.translation += translation_delta;
}
