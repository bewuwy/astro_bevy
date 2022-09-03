use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;

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
            .insert(Collider::cuboid(16.0, 16.0 * self.height as f32))
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
                            -((self.height as f32 - 1.) * 16.0) + i as f32 * 32.0,
                            1.0,
                        ),
                        ..Default::default()
                    });
                }
            });
    }
}
