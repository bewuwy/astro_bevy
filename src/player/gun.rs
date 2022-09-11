// TODO: add collision to gun

use bevy::prelude::*;

use crate::player::Player;

#[derive(Component, Clone)]
pub struct Gun {
    texture: Handle<Image>,
    pub bullet_offset: Vec2,
}

impl Gun {
    pub fn new(texture: Handle<Image>, bullet_offset: Vec2) -> Self {
        Self {
            texture,
            bullet_offset,
        }
    }

    pub fn spawn(&self, x: f32, y: f32, commands: &mut Commands) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: self.texture.clone(),
                transform: Transform::from_xyz(x, y, 20.0),
                ..Default::default()
            })
            .insert(self.clone());
    }
}

pub fn gun_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    player_query: Query<(&Player, &Transform), Without<Gun>>,
    mut gun_query: Query<(&Gun, &mut Transform)>,

    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get window
    let window = windows.get_primary().unwrap();
    // get camera
    let (camera, camera_transform) = q_camera.single();

    // get player
    for (_, player_transform) in player_query.iter() {
        // spawn gun if not already spawned
        if gun_query.iter().count() == 0 {
            println!("spawning gun");

            Gun::new(asset_server.load("gun1.png"), Vec2::new(46.0, 2.0)).spawn(
                player_transform.translation.x,
                player_transform.translation.y - 7.0,
                &mut commands,
            );
        }

        for (_, mut gun_transform) in gun_query.iter_mut() {
            // move gun to player
            gun_transform.translation.x = player_transform.translation.x;
            gun_transform.translation.y = player_transform.translation.y - 7.0;

            // rotate gun to mouse
            let mouse_pos =
                crate::input_manager::get_mouse_world_pos(window, camera, camera_transform);

            let delta_x = mouse_pos.x - player_transform.translation.x;
            let delta_y = mouse_pos.y - player_transform.translation.y;

            let rotation = delta_y.atan2(delta_x);

            // flip gun if mouse is on left side of player
            if delta_x < 0.0 {
                gun_transform.scale.y = -1.0;
            } else {
                gun_transform.scale.y = 1.0;
            }

            gun_transform.rotation = Quat::from_rotation_z(rotation);
        }
    }
}
