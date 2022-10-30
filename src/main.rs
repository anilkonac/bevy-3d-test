use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    // ecs::schedule::ReportExecutionOrderAmbiguities,
    prelude::*,
};

use bevy_3d_test::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .init_resource::<ReportExecutionOrderAmbiguities>()
        .run();
}
