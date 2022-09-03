use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::config::*;

fn bullet_system(mut commands: Commands, mut bullet_query: Query<(Entity, &Transform, &Bullet)>) {
    // despawn bullet if off screen
    for (entity, transform, _) in bullet_query.iter_mut() {
        if transform.translation.x < -WINDOW_WIDTH {
            commands.entity(entity).despawn();
        }
        if transform.translation.x > WINDOW_WIDTH {
            commands.entity(entity).despawn();
        }
        if transform.translation.y < -WINDOW_HEIGHT {
            commands.entity(entity).despawn();
        }
        if transform.translation.y > WINDOW_HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Clone)]
pub struct Bullet {
    speed: f32,
    texture: Handle<Image>,
}

impl Bullet {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            speed: 1300.0,
            texture,
        }
    }

    pub fn spawn(&self, x: f32, y: f32, direction: bool, commands: &mut Commands) {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(5.0))
            .insert(Velocity::linear(vec2(
                match direction {
                    true => 1.0,
                    false => -1.0,
                } * self.speed,
                0.0,
            )))
            .insert(GravityScale(0.0))
            .insert_bundle(SpriteBundle {
                texture: self.texture.clone(),
                transform: Transform::from_xyz(x, y, 5.0),
                ..Default::default()
            })
            .insert(Ccd::enabled())
            .insert(CollGroupsConfig::bullet())
            .insert(self.clone());
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_system);
    }
}
