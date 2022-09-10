use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::config::*;
use crate::entity::*;

fn player_system(
    mut player_query: Query<(
        &mut Player,
        &mut Velocity,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,

    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();

    for (mut player, mut player_vel, mut player_sprite, mut player_transform) in
        player_query.iter_mut()
    {
        // check if player dead
        if player.dead {
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
        let window = windows.get_primary().unwrap();

        if buttons.just_pressed(MouseButton::Left) {
            if let Some(mouse_pos) = window.cursor_position() {
                // get mouse position in world space
                // get the size of the window
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);

                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (mouse_pos / window_size) * 2.0 - Vec2::ONE;

                // matrix for undoing the projection and camera transform
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();

                // use it to convert ndc to world-space coordinates
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                // reduce it to a 2D value
                let world_pos: Vec2 = world_pos.truncate();

                // calculate vector of length 1 to mouse position
                let player_pos = Vec2::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                );
                let direction = (world_pos - player_pos).normalize();

                Bullet::new(asset_server.load("bullet/player.png")).spawn(
                    player_transform.translation.x,
                    player_transform.translation.y - 7.0,
                    direction,
                    &mut commands,
                );
            }
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Player {
    start_coords: Vec2,
    pub dead: bool,
    speed: f32,
    direction: SpriteDirection,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            start_coords: Vec2::new(x, y),
            dead: false,
            speed: 300.0,
            direction: SpriteDirection::Left,
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
}

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
                    transform: Transform::from_xyz(x, y, Z_INDEX_PLAYER),
                    ..Default::default()
                },
                rigid_body: RigidBody::Dynamic,
                velocity: Velocity::zero(),
                collider: Collider::compound(vec![
                    (Vec2::new(0.0, 0.0), 0.0, Collider::ball(10.0)),
                    (Vec2::new(0.0, -10.0), 0.0, Collider::cuboid(5.0, 5.0)),
                ]),
                gravity: GravityScale(0.0),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                coll_groups: CollGroupsConfig::player(),
            },
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_system);
    }
}
