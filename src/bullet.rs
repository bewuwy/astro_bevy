use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;
use crate::entity::enemy::Enemy;
use crate::entity::player::Player;

fn bullet_system(
    mut commands: Commands,

    mut bullet_query: Query<(Entity, &Transform, &Bullet)>,
    enemy_query: Query<(Entity, &Enemy)>,
    mut player_query: Query<&mut Player>,

    mut bullets_collision: EventReader<CollisionEvent>,
) {
    // despawn bullet if off screen
    for (entity, transform, _) in bullet_query.iter_mut() {
        if transform.translation.x < -(2.0 * WINDOW_WIDTH) {
            commands.entity(entity).despawn();
        }
        if transform.translation.x > 2.0 * WINDOW_WIDTH {
            commands.entity(entity).despawn();
        }
        if transform.translation.y < -WINDOW_HEIGHT {
            commands.entity(entity).despawn();
        }
        if transform.translation.y > WINDOW_HEIGHT {
            commands.entity(entity).despawn();
        }
    }

    // despawn bullet if it hit anything
    for collision in bullets_collision.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision {
            let mut bullet_entity = *e1;
            let mut other_entity = *e2;

            if let Ok((entity, _, _)) = bullet_query.get(*e1) {
                bullet_entity = entity;
                other_entity = *e2;
            }
            if let Ok((entity, _, _)) = bullet_query.get(*e2) {
                bullet_entity = entity;
                other_entity = *e1;
            }

            // check if player bullet hit enemy
            if let Ok((_, _, bullet)) = bullet_query.get(bullet_entity) {
                if let BulletType::Player = bullet.type_ {
                    if let Ok((_, _)) = enemy_query.get(other_entity) {
                        // despawn enemy
                        commands.entity(other_entity).despawn();
                    }
                }
            }

            // check if enemy bullet hit player
            if let Ok((_, _, bullet)) = bullet_query.get(bullet_entity) {
                if let BulletType::Enemy = bullet.type_ {
                    if let Ok(mut player) = player_query.get_mut(other_entity) {
                        player.dead = true;
                    }
                }
            }

            // despawn bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

#[derive(Component, Clone)]
pub struct Bullet {
    speed: f32,
    texture: Handle<Image>,
    type_: BulletType,
}

impl Bullet {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            speed: 1300.0,
            texture,
            type_: BulletType::Player,
        }
    }

    pub fn with_type(mut self, type_: BulletType) -> Self {
        self.type_ = type_;
        self
    }

    pub fn spawn(&self, x: f32, y: f32, direction: Vec2, commands: &mut Commands) {
        let coll_group = match self.type_ {
            BulletType::Player => CollGroupsConfig::bullet_player(),
            BulletType::Enemy => CollGroupsConfig::bullet_enemy(),
        };

        let velocity = direction * self.speed;
        // get rotation from vector
        let rotation = velocity.y.atan2(velocity.x);

        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(7.5, 2.0))
            .insert(Velocity::linear(velocity))
            .insert(GravityScale(0.0))
            .insert_bundle(SpriteBundle {
                texture: self.texture.clone(),
                transform: Transform::from_xyz(x, y, Z_INDEX_BULLET)
                    .with_rotation(Quat::from_rotation_z(rotation)),

                ..Default::default()
            })
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Ccd::enabled())
            .insert(coll_group)
            .insert(self.clone());
    }
}

#[derive(Clone, Copy)]
pub enum BulletType {
    Player,
    Enemy,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bullet_system);
    }
}
