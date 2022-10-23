use bevy::prelude::*;

fn hello() {
    println!("Hello World!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello)
        .run();
}
