use bevy::{pbr::DirectionalLightShadowMap, prelude::*, window::close_when_requested};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier3d::prelude::*;

mod player;
use player::PlayerPlugin;

const COLOR_BACKGROUND: &str = "87CEEB"; // Sky Blue
const COLOR_CUBE: &str = "FE4A49"; // Tart Orange
const COLOR_GROUND: &str = "586A6A"; // Deep Space Sparkle

const HALF_SIZE_GROUND: f32 = 7.5;
const HALF_SIZE_CUBE: f32 = 0.5;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum AppState {
    InGame,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex(COLOR_BACKGROUND).unwrap()))
            .insert_resource(Msaa::default())
            .insert_resource(DirectionalLightShadowMap::default())
            .add_plugin(EguiPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(PlayerPlugin)
            .add_state(AppState::InGame)
            .add_startup_system(setup)
            .add_system(grab_mouse.label("grab_mouse").before(close_when_requested))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(ui_graphics));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // Grab mouse
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

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

    // Create light
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
    key: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        let window = windows.get_primary_mut().unwrap();

        match app_state.current() {
            AppState::InGame => {
                window.set_cursor_visibility(true);
                window.set_cursor_lock_mode(false);
                app_state.set(AppState::Menu).unwrap();
            }
            AppState::Menu => {
                window.set_cursor_visibility(false);
                window.set_cursor_lock_mode(true);
                app_state.set(AppState::InGame).unwrap();
            }
        }
    }
}

fn ui_graphics(
    mut egui_context: ResMut<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut shadop_map: ResMut<DirectionalLightShadowMap>,
) {
    egui::Window::new("Graphics").show(egui_context.ctx_mut(), |ui| {
        let mut msaa_active = msaa.samples > 1;
        ui.checkbox(&mut msaa_active, "MSAA");
        if msaa_active {
            msaa.samples = 4;
        } else {
            msaa.samples = 1;
        }
        ui.end_row();
        ui.add(
            egui::Slider::new(&mut shadop_map.size, 2048..=2048 * 10)
                .step_by(2048.0)
                .text("Shadow Map Size"),
        );
    });
}
