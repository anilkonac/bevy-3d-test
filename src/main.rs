use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    // ecs::schedule::ReportExecutionOrderAmbiguities,
    input::mouse::MouseMotion,
    prelude::*,
    time::FixedTimestep,
    window::close_when_requested,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 3.0;
const KEYS_FORWARD: [KeyCode; 2] = [KeyCode::W, KeyCode::Up];
const KEYS_BACKWARD: [KeyCode; 2] = [KeyCode::S, KeyCode::Down];
const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];
const KEYS_LEFT: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];

// Resource
struct MouseGrabbed(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(MouseGrabbed(false))
        .add_startup_system(setup)
        .add_system(grab_mouse.before(close_when_requested))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_look_system.before(player_movement_system))
                .with_system(player_movement_system),
        )
        // .init_resource::<ReportExecutionOrderAmbiguities>()
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
    mut mouse_grabbed: ResMut<MouseGrabbed>,
) {
    // Grab mouse
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);
    mouse_grabbed.0 = true;

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

fn player_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }
    // println!("{:?}", delta);

    let mut transform = query.single_mut();
    transform.rotate_y(-delta.x * TIME_STEP);
    transform.rotate_local_x(-delta.y * TIME_STEP);
    // transform.rotate_local(Quat::from_euler(
    //     EulerRot::YXZ,
    //     -delta.x.to_radians(),
    //     -delta.y.to_radians(),
    //     0.0,
    // ));
}

fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut mouse_grabbed: ResMut<MouseGrabbed>,
) {
    let window = windows.get_primary_mut().unwrap();
    if !mouse_grabbed.0 && mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
        mouse_grabbed.0 = true;
    } else if mouse_grabbed.0 && key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
        mouse_grabbed.0 = false;
    }
}
