use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

mod bullet;
mod config;
mod enemy;
mod player;
mod wall;

use bullet::BulletPlugin;
use config::*;
use enemy::{Enemy, EnemyPlugin};
use player::PlayerPlugin;
use wall::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // window setup
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            // resizable: false,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            // present_mode: PresentMode::AutoVsync,
            ..default()
        })
        // .insert_resource(ClearColor(Color::NONE))
        .insert_resource(ClearColor(Color::rgb(
            BACKGROUND_COLOR[0] / 255.0,
            BACKGROUND_COLOR[1] / 255.0,
            BACKGROUND_COLOR[2] / 255.0,
        )))
        // // rapier
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // debug
        .add_plugin(RapierDebugRenderPlugin::default())
        // systems
        .add_startup_system(setup)
        // plugins
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            // far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(WINDOW_HEIGHT),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn walls
    Wall::new(10, asset_server.load("wall.png")).spawn(-200.0, 0.0, &mut commands);
    Wall::new(8, asset_server.load("wall.png")).spawn(200.0, -20.0, &mut commands);

    // Spawn enemy
    Enemy::new().spawn(
        -100.0,
        -100.0,
        asset_server.load("enemy.png"),
        &mut commands,
    );
}
