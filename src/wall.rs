use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::config::*;

static WALL_SPRITE_SIZE: f32 = 32.0;

pub struct WallEntity {
    height: u32,
    texture: Handle<Image>,
}

impl WallEntity {
    pub fn new(height: u32, texture: Handle<Image>) -> Self {
        WallEntity { height, texture }
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

pub fn spawn_wall_colliders(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
) {

    for (coords, parent) in wall_query.iter() {

        let x = (coords.x as f32 + 0.5) * WALL_SPRITE_SIZE;
        let y = (coords.y as f32 + 0.5) * WALL_SPRITE_SIZE;

        commands
            .spawn()
            .insert_bundle(TransformBundle {
                local: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                WALL_SPRITE_SIZE / 2.0,
                WALL_SPRITE_SIZE / 2.0,
            ));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
