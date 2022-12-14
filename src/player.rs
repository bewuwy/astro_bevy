pub mod gun;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::*;
use crate::entity::bullet::*;
use crate::entity::*;

use self::gun::{gun_system, Gun};

#[allow(clippy::type_complexity)]
fn player_system(
    mut player_query: Query<(
        &mut Player,
        &mut Velocity,
        &mut ExternalImpulse,
        &mut CollisionGroups,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    gun_query: Query<(&Gun, &Transform), Without<Player>>,

    (keyboard_input, buttons, windows, asset_server, time): (
        Res<Input<KeyCode>>,
        Res<Input<MouseButton>>,
        Res<Windows>,
        Res<AssetServer>,
        Res<Time>,
    ),

    mut commands: Commands,
) {
    let (camera, camera_transform) = q_camera.single();

    for (
        mut player,
        mut player_vel,
        mut player_impulse,
        mut player_coll_groups,
        mut player_sprite,
        mut player_transform,
    ) in player_query.iter_mut()
    {
        // update player z index in a stupid way, because ldtk doesn't support z index (i think)
        if player_transform.translation.z != 10.0 {
            player_transform.translation.z = 10.0;
        }

        // check if player dead
        if player.dead && !player.immortal {
            // teleport player
            player_transform.translation.x = player.start_coords.x;
            player_transform.translation.y = player.start_coords.y;

            // reset player velocity
            player_vel.linvel = Vec2 { x: 0.0, y: 0.0 };

            println!("You ded.");
            player.dead = false;
        }

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

        // dash
        const PLAYER_DASH_SPEED: f32 = 15.0;
        player.dash_timer.tick(time.delta());
        player.dash_cooldown.tick(time.delta());

        if player.dash_cooldown.finished()
            && !player.dashing
            && keyboard_input.just_pressed(KeyCode::Space)
        {
            player.dashing = true;
            player.dash_timer.reset();
        }

        if player.dashing && !player.dash_timer.finished() {
            let direction = match player.direction {
                SpriteDirection::Up => vec2(0.0, 1.0),
                SpriteDirection::Down => vec2(0.0, -1.0),
                SpriteDirection::Left => vec2(-1.0, 0.0),
                SpriteDirection::Right => vec2(1.0, 0.0),
            };

            player_impulse.impulse = direction * PLAYER_DASH_SPEED;

            player_coll_groups.filters = 0b10000;
        }
        if player.dash_timer.just_finished() {
            player.dashing = false;
            player_coll_groups.filters = CollGroupsConfig::player().filters;

            player.dash_cooldown.reset();
        }

        // update sprite
        match player.direction {
            SpriteDirection::Right => player_sprite.index = 0,
            SpriteDirection::Left => player_sprite.index = 1,
            SpriteDirection::Up => player_sprite.index = 2,
            SpriteDirection::Down => player_sprite.index = 3,
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }
        player_vel.linvel = move_delta * player.speed;

        // shooting
        let window = windows.get_primary().unwrap();

        if buttons.just_pressed(MouseButton::Left) {
            let mouse_pos =
                crate::input_manager::get_mouse_world_pos(window, camera, camera_transform);

            // calculate vector of length 1 to mouse position
            let player_pos = Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );
            let direction = (mouse_pos - player_pos).normalize();

            // get gun transform
            let (gun, gun_transform) = gun_query.single();

            let mut bullet_spawn =
                Vec2::new(gun_transform.translation.x, gun_transform.translation.y);
            // adjust bullet spawn position
            bullet_spawn += direction * gun.bullet_offset.x;
            bullet_spawn.y += gun.bullet_offset.y;

            Bullet::new(asset_server.load("bullet/player.png")).spawn(
                bullet_spawn.x,
                bullet_spawn.y,
                direction,
                &mut commands,
            );
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Player {
    start_coords: Vec2,
    pub dead: bool,
    dashing: bool,
    dash_timer: Timer,
    dash_cooldown: Timer,
    speed: f32,
    direction: SpriteDirection,
    immortal: bool,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            start_coords: Vec2::new(x, y),
            dead: false,
            dashing: false,
            dash_timer: Timer::from_seconds(0.3, false),
            dash_cooldown: Timer::from_seconds(0.3, false),
            speed: 300.0,
            direction: SpriteDirection::Left,
            immortal: false,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Clone, Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    worldly: Worldly,
    #[bundle]
    entity_bundle: EntityBundle,
    external_impulse: ExternalImpulse,
    ccd: Ccd,
}

// spawning player from LDtk
impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        textures: &mut Assets<TextureAtlas>,
    ) -> PlayerBundle {
        let x = entity.px.x as f32;
        let y = WINDOW_HEIGHT - entity.px.y as f32;

        // spawn player - return bundle
        PlayerBundle {
            player: Player::new(x, y),
            worldly: Worldly::from_entity_info(entity),
            entity_bundle: EntityBundle {
                sprite_bundle: SpriteSheetBundle {
                    texture_atlas: textures.add(TextureAtlas::from_grid(
                        asset_server.load("player/sheet.png"),
                        Vec2 { x: 32.0, y: 32.0 },
                        4,
                        1,
                    )),
                    transform: Transform::from_xyz(x, y, 100.0),
                    ..Default::default()
                },
                rigid_body: RigidBody::Dynamic,
                velocity: Velocity::zero(),
                collider: Collider::compound(vec![(
                    Vec2::new(0.0, -2.0),
                    0.0,
                    Collider::cuboid(7.0, 14.0),
                )]),
                gravity: GravityScale(0.0),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                coll_groups: CollGroupsConfig::player(),
            },
            external_impulse: ExternalImpulse::default(),
            ccd: Ccd { enabled: true },
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_system).add_system(gun_system);
    }
}
