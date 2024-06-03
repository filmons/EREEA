mod camera;
mod debug;
mod movement;
mod robot;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use robot::RobotPlugin;

fn main() {
    App::new()
        // Bevy fonctions
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(), //Blanche
            brightness: 0.75
        })
        .add_plugins(DefaultPlugins)

        // Nos fonctions
        .add_plugins(RobotPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
