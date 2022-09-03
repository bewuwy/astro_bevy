use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::config::*;

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Create the player */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        // .insert(Collider::ball(20.0))
        .insert(Collider::compound(vec![
            (Vec2::new(0.0, 0.0), 0.0, Collider::ball(20.0)),
            (Vec2::new(0.0, -20.0), 0.0, Collider::cuboid(10.0, 10.0)),
        ]))
        // .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero())
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player::new())
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, 300.0, 5.0),
            ..Default::default()
        })
        .insert(CollGroupsConfig::player());
}

fn player_system(
    mut player_query: Query<(&mut Player, &mut Velocity, &mut Sprite, &Transform)>,

    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let (mut player, mut player_vel, mut player_sprite, player_transform) =
        player_query.single_mut();

    // movement
    let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
    let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    if x_axis != 0 {
        player.direction = right;

        player_sprite.flip_x = !player.direction;
    }

    let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
    if move_delta != Vec2::ZERO {
        move_delta /= move_delta.length();
    }

    // Update the velocity on the rigid_body_component,
    // the bevy_rapier plugin will update the Sprite transform.
    player_vel.linvel = move_delta * player.speed;

    // shooting
    if keyboard_input.just_pressed(KeyCode::Space) {
        Bullet::new(asset_server.load("bullet.png")).spawn(
            player_transform.translation.x,
            player_transform.translation.y,
            player.direction,
            &mut commands,
        );
    }
}

#[derive(Component)]
struct Player {
    speed: f32,
    direction: bool, // true = right, false = left
}

impl Player {
    fn new() -> Self {
        Self {
            speed: 500.0,
            direction: false,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup)
            .add_system(player_system);
    }
}
