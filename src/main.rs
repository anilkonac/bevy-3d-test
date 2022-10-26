use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    // ecs::schedule::ReportExecutionOrderAmbiguities,
    prelude::*,
    window::close_when_requested,
};
use player::PlayerPlugin;

mod player;

// Resource
struct MouseGrabbed(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(MouseGrabbed(false))
        .add_startup_system(setup)
        .add_system(grab_mouse.before(close_when_requested))
        // .init_resource::<ReportExecutionOrderAmbiguities>()
        .run();
}

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
    let cam_transform = Transform::from_xyz(-5.0, player::PLAYER_HEIGHT, -4.0).looking_at(
        Vec3 {
            x: 0.0,
            y: player::PLAYER_HEIGHT,
            z: 0.0,
        },
        Vec3::Y,
    );
    commands
        .spawn_bundle(Camera3dBundle {
            transform: cam_transform,
            ..default()
        })
        .insert(player::Player)
        .insert(player::CameraState {
            pitch: 0.0,
            yaw: cam_transform.rotation.to_euler(EulerRot::YXZ).0,
        });
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
