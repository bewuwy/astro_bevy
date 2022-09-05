use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::config::*;

static START_X: f32 = WINDOW_WIDTH/2.0;
static START_Y: f32 = 200.0;

fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    /* Create the player */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        // .insert(Collider::ball(20.0))
        .insert(Collider::compound(vec![
            (Vec2::new(0.0, 0.0), 0.0, Collider::ball(10.0)),
            (Vec2::new(0.0, -10.0), 0.0, Collider::cuboid(5.0, 5.0)),
        ]))
        // .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity::zero())
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Player::new())
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("player/sheet.png"),
                Vec2 { x: 32.0, y: 32.0 },
                4,
                1,
            )),
            transform: Transform::from_xyz(START_X, START_Y, Z_INDEX_PLAYER),
            ..Default::default()
        })
        .insert(CollGroupsConfig::player());
}

fn player_system(
    mut player_query: Query<(
        &mut Player,
        &mut Velocity,
        &mut TextureAtlasSprite,
        &Transform,
    )>,

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
        player.direction = match right {
            true => SpriteDirection::Right,
            false => SpriteDirection::Left,
        };
    }
    if y_axis != 0 {
        player.direction = match up {
            true => SpriteDirection::Up,
            false => SpriteDirection::Down,
        };
    }

    // update sprite
    match player.direction {
        SpriteDirection::Right => player_sprite.index = 0,
        SpriteDirection::Left => player_sprite.index = 1,
        SpriteDirection::Up => player_sprite.index = 2,
        SpriteDirection::Down => player_sprite.index = 3,
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
        let bullet_x = match player.direction {
            SpriteDirection::Left => player_transform.translation.x - 8.0,
            SpriteDirection::Right => player_transform.translation.x + 8.0,
            SpriteDirection::Down => player_transform.translation.x - 7.0,
            SpriteDirection::Up => player_transform.translation.x + 7.0,
        };

        Bullet::new(asset_server.load("bullet.png")).spawn(
            bullet_x,
            player_transform.translation.y - 7.0,
            player.direction,
            &mut commands,
        );
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    direction: SpriteDirection,
}

impl Player {
    fn new() -> Self {
        Self {
            speed: 500.0,
            direction: SpriteDirection::Left,
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
