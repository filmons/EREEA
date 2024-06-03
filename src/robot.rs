use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct RobotBundle {
    velocity: Velocity,
    // model: SceneBundle, //Pour la 3D
    sprite: SpriteBundle //2D
}

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_robot);
    }
}

fn spawn_robot(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(RobotBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        sprite: SpriteBundle {
            texture: asset_server.load("robot.png"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}