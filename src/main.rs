use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::prelude::*;

mod config;
mod player;
mod bullet;
mod wall;

use config::*;
use player::PlayerPlugin;
use bullet::BulletPlugin;
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
        .insert_resource(ClearColor(Color::rgb(
            BACKGROUND_COLOR[0] / 255.0,
            BACKGROUND_COLOR[1] / 255.0,
            BACKGROUND_COLOR[2] / 255.0,
        )))
        // rapier
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        // systems
        .add_startup_system(setup)
        // plugins
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add a 2D Camera
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            // far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(WINDOW_HEIGHT),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn wall
    Wall::new(10, asset_server.load("wall.png")).spawn(-200.0, 0.0, &mut commands);
}
