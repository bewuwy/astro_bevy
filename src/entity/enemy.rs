use std::time::Duration;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;
use crate::entity::bullet::{Bullet, BulletType};

use super::EntitySpriteBundle;

#[derive(Default, Component, Clone)]
pub struct Enemy {
    direction: SpriteDirection,
    last_shot: Timer,
}

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            direction: SpriteDirection::Right,
            last_shot: Timer::from_seconds(1.0, true),
        }
    }
}

fn enemy_system(
    mut enemy_query: Query<(&mut Enemy, &GlobalTransform)>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (mut enemy, transform) in enemy_query.iter_mut() {
        enemy.last_shot.tick(time.delta());

        let bullet_direction = match enemy.direction {
            SpriteDirection::Right => vec2(1.0, 0.0),
            _ => vec2(-1.0, 0.0),
        };

        if enemy.last_shot.finished() {
            // spawn bullet
            Bullet::new(asset_server.load("bullet/enemy.png"))
                .with_type(BulletType::Enemy)
                .spawn(
                    transform.translation().x,
                    transform.translation().y - 4.0,
                    bullet_direction,
                    &mut commands,
                );

            // reset timer to random value
            enemy.last_shot.reset();
            enemy
                .last_shot
                .set_duration(Duration::from_secs_f32(rand::random::<f32>() * 2.0));
        }
    }
}

#[derive(Clone, Default, Bundle)]
pub struct EnemyBundle {
    #[bundle]
    entity_bundle: EntitySpriteBundle,
    enemy: Enemy,
}

impl LdtkEntity for EnemyBundle {
    fn bundle_entity(
        entity: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> EnemyBundle {
        let mut enemy = Enemy::new();
        let mut x_flip = false;

        if let Some(rotation_field) = entity
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Rotation")
        {
            if let FieldValue::Enum(Some(rot)) = &rotation_field.value {
                match rot.as_str() {
                    "Left" => {
                        enemy.direction = SpriteDirection::Left;
                        x_flip = true
                    }
                    "Right" => enemy.direction = SpriteDirection::Right,
                    _ => {}
                }
            }
        }

        // enemy collider
        let collider = Collider::compound(vec![(
            Vec2::new(-2.0, -2.5),
            0.0,
            Collider::cuboid(11.0, 13.0),
        )]);

        EnemyBundle {
            entity_bundle: EntitySpriteBundle {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("enemy/snake.png"),
                    sprite: Sprite {
                        flip_x: x_flip,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                collider,
                rigid_body: RigidBody::Dynamic,
                coll_groups: CollGroupsConfig::enemy(),
                gravity: GravityScale(0.0),
                locked_axes: LockedAxes::ROTATION_LOCKED,
            },
            enemy,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_system);
    }
}
