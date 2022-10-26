use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

const PLAYER_SPEED: f32 = 3.0;
pub const PLAYER_HEIGHT: f32 = 1.8;
const KEYS_FORWARD: [KeyCode; 2] = [KeyCode::W, KeyCode::Up];
const KEYS_BACKWARD: [KeyCode; 2] = [KeyCode::S, KeyCode::Down];
const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];
const KEYS_LEFT: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];
const MOUSE_SENSITIVITY: f32 = 0.15;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_look_system.before(player_move_system))
            .add_system(player_move_system);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
}

fn player_move_system(
    time: Res<Time>,
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
    transform.translation += movement_direction * PLAYER_SPEED * time.delta_seconds();
}

fn player_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraState), With<Player>>,
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
