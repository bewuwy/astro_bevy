use bevy::prelude::*;

use crate::{config::*, entity::player::*};

fn camera_system(
    mut camera_query: Query<(&mut Transform, &Camera2d), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_transform in player_query.iter() {
        let mut x = player_transform.translation.x;
        let mut y = player_transform.translation.y;

        // camera constraints
        x = x.clamp(
            WINDOW_WIDTH / (CAMERA_SCALE * 2.1),
            WINDOW_WIDTH - WINDOW_WIDTH / (CAMERA_SCALE * 2.1),
        );
        y = y.clamp(
            WINDOW_HEIGHT / (CAMERA_SCALE * 2.1),
            WINDOW_HEIGHT - WINDOW_HEIGHT / (CAMERA_SCALE * 2.1),
        );

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
