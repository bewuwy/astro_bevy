use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;

static WALL_SPRITE_SIZE: f32 = 64.0;

pub struct Wall {
    height: u32,
    texture: Handle<Image>,
}

impl Wall {
    pub fn new(height: u32, texture: Handle<Image>) -> Self {
        Wall { height, texture }
    }

    pub fn spawn(&self, x: f32, y: f32, commands: &mut Commands) {
        commands
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                WALL_SPRITE_SIZE / 2.0,
                WALL_SPRITE_SIZE / 2.0 * self.height as f32,
            ))
            .insert_bundle(SpatialBundle::from_transform(Transform::from_xyz(
                x, y, 0.0,
            )))
            .insert(CollGroupsConfig::wall())
            .with_children(|parent| {
                for i in 0..self.height {
                    parent.spawn().insert_bundle(SpriteBundle {
                        texture: self.texture.clone(),
                        transform: Transform::from_xyz(
                            0.0,
                            -((self.height as f32 - 1.) * WALL_SPRITE_SIZE / 2.0)
                                + i as f32 * WALL_SPRITE_SIZE,
                            Z_INDEX_WALL,
                        ),
                        ..Default::default()
                    });
                }
            });
    }
}
