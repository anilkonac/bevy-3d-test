use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::AppState;

const PLAYER_SPEED: f32 = 3.0;
const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_HEIGHT_HEAD: f32 = 1.6;
const PLAYER_INITIAL_POS: Vec3 = Vec3::new(-5.0, PLAYER_HEIGHT / 2.0, -4.0);
const MOUSE_SENSITIVITY: f32 = 0.15;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct HeadState {
    pitch: f32,
    yaw: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_look_system.before(player_move_system)),
            )
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(player_move_system));
    }
}

fn setup_player(mut commands: Commands) {
    let transform_player = Transform::from_translation(PLAYER_INITIAL_POS)
        .looking_at(Vec3::new(0.0, PLAYER_HEIGHT / 2.0, 0.0), Vec3::Y);

    let translation_head = Transform::from_translation(Vec3::new(
        0.0,
        PLAYER_HEIGHT / 4.0 + PLAYER_HEIGHT_HEAD / 2.0,
        0.0,
    ));

    commands
        .spawn()
        // .insert(transform_player)
        .insert_bundle(TransformBundle {
            local: transform_player,
            ..default()
        })
        .insert(Player)
        .with_children(|parent| {
            parent
                .spawn_bundle(Camera3dBundle {
                    transform: translation_head,
                    ..default()
                })
                .insert(HeadState {
                    pitch: 0.0,
                    yaw: 0.0,
                });
        });
}

fn player_move_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
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

    let mut transform = query.single_mut();

    //  Calculate movement direction
    let movement_direction = movement_axes.z * transform.forward()
        + movement_axes.x * transform.right()
        + movement_axes.y * transform.up();

    let movement_direction = movement_direction.normalize();

    // Apply translation
    transform.translation += movement_direction * PLAYER_SPEED * time.delta_seconds();
}

fn player_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut HeadState), With<HeadState>>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let (mut transform, mut cam_state) = query.single_mut();

    let mut yaw = cam_state.yaw;
    let mut pitch = cam_state.pitch;
    yaw -= (delta.x * MOUSE_SENSITIVITY).to_radians();
    pitch -= (delta.y * MOUSE_SENSITIVITY).to_radians();
    pitch = pitch.clamp(0.9 * -FRAC_PI_2, 0.9 * FRAC_PI_2);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    cam_state.yaw = yaw;
    cam_state.pitch = pitch;
}
