use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// function to spawn specific level from ldtk
pub fn spawn_levels(
    mut commands: Commands,
    // mut level_sets: Query<&mut LevelSet>,
    asset_server: Res<AssetServer>,
    // level_iid: String,
) {
    let mut iids = HashSet::new();

    iids.insert("28c31d50-2a00-11ed-9c42-3190aff26295".to_string());
    iids.insert("d10a4e50-2a00-11ed-8634-bb1f07d1b82b".to_string());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk"),
        level_set: LevelSet { iids },
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    // let mut iids2 = HashSet::new();

    // // iids.insert("28c31d50-2a00-11ed-9c42-3190aff26295".to_string());
    // iids2.insert("d10a4e50-2a00-11ed-8634-bb1f07d1b82b".to_string());

    // commands.spawn_bundle(LdtkWorldBundle {
    //     ldtk_handle: asset_server.load("world.ldtk"),
    //     level_set: LevelSet { iids: iids2 },
    //     transform: Transform::from_xyz(640., 0., 0.),
    //     ..Default::default()
    // });
}
