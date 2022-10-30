use bevy::{prelude::*, window::close_when_requested};

mod player;
use player::PlayerPlugin;

const COLOR_BACKGROUND: &str = "87CEEB"; // Sky Blue
const COLOR_CUBE: &str = "FE4A49"; // Tart Orange
const COLOR_GROUND: &str = "586A6A"; // Deep Space Sparkle

// Resource
struct MouseGrabbed(bool);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex(COLOR_BACKGROUND).unwrap())) // Sky Blue
            .add_plugin(PlayerPlugin)
            .insert_resource(MouseGrabbed(false))
            .add_startup_system(setup)
            .add_system(grab_mouse.before(close_when_requested));
    }
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
        material: materials.add(Color::hex(COLOR_GROUND).unwrap().into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::hex(COLOR_CUBE).unwrap().into()),
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
