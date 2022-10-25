use bevy::{prelude::*, time::FixedTimestep};

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 3.0;
const KEYS_FORWARD: [KeyCode; 2] = [KeyCode::W, KeyCode::Up];
const KEYS_BACKWARD: [KeyCode; 2] = [KeyCode::S, KeyCode::Down];
const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];
const KEYS_LEFT: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system),
        )
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
    let mut movement_factor_forward = 0.0;
    let mut movement_factor_right = 0.0;
    if keyboard_input.any_pressed(KEYS_FORWARD) {
        movement_factor_forward += 1.0
    }
    if keyboard_input.any_pressed(KEYS_BACKWARD) {
        movement_factor_forward -= 1.0;
    }
    if keyboard_input.any_pressed(KEYS_RIGHT) {
        movement_factor_right += 1.0;
    }
    if keyboard_input.any_pressed(KEYS_LEFT) {
        movement_factor_right += -1.0;
    }

    if (movement_factor_forward == 0.0) && (movement_factor_right == 0.0) {
        return;
    }

    let mut transform = query.single_mut();

    //  Calculate movement direction
    let movement_direction =
        movement_factor_forward * transform.forward() + movement_factor_right * transform.right();

    let movement_direction = movement_direction.normalize();

    // Apply translation
    transform.translation += movement_direction * PLAYER_SPEED * TIME_STEP;
}
