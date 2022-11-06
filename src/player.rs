use bevy::{input::mouse::MouseMotion, prelude::*, window::close_when_requested};
use std::f32::consts::FRAC_PI_2;

use crate::AppState;

const PLAYER_SPEED: f32 = 3.0;
const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_HEAD_ALT: f32 = 1.6;
const PLAYER_INITIAL_POS: Vec3 = Vec3::new(-5.0, PLAYER_HEIGHT / 2.0, -4.0);
const HEAD_SIZE: f32 = (PLAYER_HEIGHT - PLAYER_HEAD_ALT) * 2.0;
const TORSO_WIDTH: f32 = HEAD_SIZE * 2.0;
const TORSO_HEIGHT: f32 = PLAYER_HEAD_ALT / 2.0;
const TORSO_ALT_RELATIVE: f32 = 0.0;
const MOUSE_SENSITIVITY: f32 = 100.0;

const COLOR_PLAYER_BODY: &str = "A1C084"; // Olivine
const COLOR_PLAYER_HEAD: &str = "F6F740"; // Maximum Yellow

// To tag player entity
#[derive(Component)]
struct Player;

// To specify which entities should rotate
#[derive(Component)]
struct Rotator;

#[derive(Component, Default)]
struct HeadState {
    pitch: f32,
    yaw: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player.after("main_setup"))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(
                        player_look_system
                            .before(player_move_system)
                            .after("grab_mouse")
                            .before(close_when_requested),
                    )
                    .with_system(player_move_system),
            );
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform_player = Transform::from_translation(PLAYER_INITIAL_POS)
        .looking_at(Vec3::new(0.0, PLAYER_HEIGHT / 2.0, 0.0), Vec3::Y);

    let transform_head = Transform::from_xyz(0.0, PLAYER_HEAD_ALT - PLAYER_HEIGHT / 2.0, 0.0);

    let transform_third_person_cam =
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    let player = commands
        .spawn_bundle(SpatialBundle {
            transform: transform_player,
            ..default()
        })
        .insert(Player)
        .insert(Rotator)
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(0.0, TORSO_ALT_RELATIVE, 0.0),
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    TORSO_WIDTH,
                    TORSO_HEIGHT,
                    HEAD_SIZE,
                ))),
                material: materials.add(Color::hex(COLOR_PLAYER_BODY).unwrap().into()),
                ..default()
            });
        })
        .id();

    let head = commands
        .spawn_bundle(PbrBundle {
            transform: transform_head,
            mesh: meshes.add(Mesh::from(shape::Cube::new(HEAD_SIZE))),
            material: materials.add(Color::hex(COLOR_PLAYER_HEAD).unwrap().into()),
            ..default()
        })
        .insert(HeadState::default())
        .insert(Rotator)
        .with_children(|parent| {
            parent.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, -HEAD_SIZE / 2.0),
                camera: Camera {
                    is_active: false,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    let third_person_cam = commands
        .spawn_bundle(Camera3dBundle {
            transform: transform_third_person_cam,
            ..default()
        })
        .id();

    commands.entity(head).push_children(&[third_person_cam]);
    commands.entity(player).push_children(&[head]);
}

fn player_move_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    query_player: Query<Entity, With<Player>>,
    query_head: Query<Entity, With<HeadState>>,
    mut query_transforms: Query<&mut Transform, With<Rotator>>,
    mut query_head_state: Query<&mut HeadState>,
) {
    let mut movement_axes = Vec3::ZERO;
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement_axes.z += 1.0
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement_axes.z -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement_axes.x += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement_axes.x += -1.0;
    }
    if keyboard_input.any_pressed([KeyCode::E, KeyCode::RShift]) {
        movement_axes.y += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Q, KeyCode::RControl]) {
        movement_axes.y += -1.0;
    }

    if movement_axes == Vec3::ZERO {
        return;
    }

    let entity_player = query_player.single();

    rotate_player_to_head_yaw(
        entity_player,
        &query_head,
        &mut query_transforms,
        &mut query_head_state,
    );

    translate_player(entity_player, &mut query_transforms, movement_axes, &time);
}

fn rotate_player_to_head_yaw(
    entity_player: Entity,
    query_head: &Query<Entity, With<HeadState>>,
    query_transforms: &mut Query<&mut Transform, With<Rotator>>,
    query_head_state: &mut Query<&mut HeadState>,
) {
    let mut head_state = query_head_state.single_mut();
    if head_state.yaw == 0.0 {
        return;
    }

    // Rotate player body
    let mut transform_player = query_transforms.get_mut(entity_player).unwrap();
    transform_player.rotate_y(head_state.yaw);

    // Reset head yaw
    let entity_head = query_head.single();
    let mut transform_head = query_transforms.get_mut(entity_head).unwrap();
    transform_head.rotation = Quat::from_rotation_x(head_state.pitch);

    head_state.yaw = 0.0;
}

fn translate_player(
    entity_player: Entity,
    query_transforms: &mut Query<&mut Transform, With<Rotator>>,
    movement_axes: Vec3,
    time: &Res<Time>,
) {
    let mut transform_player = query_transforms.get_mut(entity_player).unwrap();

    //  Calculate movement direction
    let movement_direction = movement_axes.z * transform_player.forward()
        + movement_axes.x * transform_player.right()
        + movement_axes.y * transform_player.up();

    let movement_direction = movement_direction.normalize();

    // Apply translation
    transform_player.translation += movement_direction * PLAYER_SPEED * time.delta_seconds();
}

fn player_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut HeadState), With<HeadState>>,
    windows: Res<Windows>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let (mut transform, mut head_state) = query.single_mut();

    let window = windows.get_primary().unwrap();

    let mut yaw = head_state.yaw;
    let mut pitch = head_state.pitch;
    yaw -= ((delta.x / window.width()) * MOUSE_SENSITIVITY).to_radians();
    pitch -= ((delta.y / window.height()) * MOUSE_SENSITIVITY).to_radians();
    pitch = pitch.clamp(-0.9 * FRAC_PI_2, 0.9 * FRAC_PI_2);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    head_state.yaw = yaw;
    head_state.pitch = pitch;
}
