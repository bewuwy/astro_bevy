use bevy_rapier2d::prelude::*;

// game config
pub static GAME_NAME: &str = "Astro bevy";
pub static WINDOW_WIDTH: f32 = 1280.0;
pub static WINDOW_HEIGHT: f32 = 960.0;

pub static BACKGROUND_COLOR: [f32; 3] = [124.0, 132.0, 131.0];

// z-index config
pub static Z_INDEX_PLAYER: f32 = 10.0;
pub static Z_INDEX_BULLET: f32 = 5.0;
pub static Z_INDEX_ENEMY: f32 = 6.0;
pub static Z_INDEX_WALL: f32 = 1.0;

// rapier config
pub struct CollGroupsConfig;

impl CollGroupsConfig {
    // pain...

    pub fn player() -> CollisionGroups {
        // Group 0, interacts with groups 2, 3 and 4
        CollisionGroups::new(0b00001, 0b11100)
    }

    pub fn bullet_player() -> CollisionGroups {
        // Group 1, interacts with group 3 and 4
        CollisionGroups::new(0b00010, 0b11000)
    }

    // TODO: make bullet_enemy interact with bullet_player
    pub fn bullet_enemy() -> CollisionGroups {
        // Group 2, interacts with group 0 and 4
        CollisionGroups::new(0b00100, 0b10001)
    }

    pub fn enemy() -> CollisionGroups {
        // Group 3, interacts with group 0, 1 and 4
        CollisionGroups::new(0b01000, 0b10011)
    }

    pub fn wall() -> CollisionGroups {
        // Group 4, interacts with group 0, 1, 2, 3
        CollisionGroups::new(0b10000, 0b01111)
    }
}

// utility

#[derive(PartialEq, Clone, Copy)]
pub enum SpriteDirection {
    Left,
    Right,
    Up,
    Down,
}
