use bevy::prelude::*;

use crate::player::*;

fn camera_system(
    mut camera_query: Query<(&mut Transform, &Camera2d), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_transform in player_query.iter() {
        let x = player_transform.translation.x;
        let y = player_transform.translation.y;

        for (mut transform, _) in camera_query.iter_mut() {
            transform.translation = Vec3::new(x, y, 1000.0);
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_system);
    }
}
