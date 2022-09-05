use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod bullet;
mod config;
mod enemy;
mod player;
mod wall;

use bullet::BulletPlugin;
use config::*;
use enemy::*;
use player::PlayerPlugin;
use wall::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ldtk
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_entity::<EnemyBundle>("Snake_Enemy")
        .add_system(spawn_wall_colliders)
        // window setup
        .insert_resource(WindowDescriptor {
            title: GAME_NAME.to_string(),
            // resizable: false,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            // present_mode: PresentMode::AutoVsync,
            ..default()
        })
        // pixel art camera setup
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        // background colour
        .insert_resource(ClearColor(Color::rgb(
            BACKGROUND_COLOR[0] / 255.0,
            BACKGROUND_COLOR[1] / 255.0,
            BACKGROUND_COLOR[2] / 255.0,
        )))
        // rapier
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
            scaling_mode: ScalingMode::FixedVertical(WINDOW_HEIGHT),
            ..Default::default()
        },
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 1000.0),
        ..Default::default()
    });

    // Spawn from LDtk
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk.ldtk"),
        ..Default::default()
    });
}
