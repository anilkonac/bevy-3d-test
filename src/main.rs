use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    // ecs::schedule::ReportExecutionOrderAmbiguities,
    prelude::*,
    // window::PresentMode,
};

use bevy_3d_test::GamePlugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // window: WindowDescriptor {
            //     present_mode: PresentMode::AutoNoVsync,
            //     ..default()
            // },
            ..default()
        }))
        .add_plugin(GamePlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .init_resource::<ReportExecutionOrderAmbiguities>()
        .run();
}
