use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod bullet;
pub mod enemy;

#[derive(Clone, Default, Bundle)]
pub struct EntityBundle {
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub coll_groups: CollisionGroups,
    pub gravity: GravityScale,
    pub locked_axes: LockedAxes,
}

#[derive(Clone, Default, Bundle)]
pub struct EntitySpriteBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub coll_groups: CollisionGroups,
    pub gravity: GravityScale,
    pub locked_axes: LockedAxes,
}
