use bevy_rapier2d::prelude::*;

// game config
pub static GAME_NAME: &str = "Astro bevy";
pub static WINDOW_WIDTH: f32 = 1280.0;
pub static WINDOW_HEIGHT: f32 = 720.0;

pub static BACKGROUND_COLOR: [f32; 3] = [124.0, 132.0, 131.0];

// rapier config
pub struct CollGroupsConfig;

impl CollGroupsConfig {
    pub fn player() -> CollisionGroups {
        // Group 0, interacts with group 2
        CollisionGroups::new(0b001, 0b100)
    }

    pub fn bullet() -> CollisionGroups {
        // Group 1, interacts with group 2
        CollisionGroups::new(0b010, 0b100)
    }

    pub fn wall() -> CollisionGroups {
        // Group 2, interacts with group 0 and 1
        CollisionGroups::new(0b100, 0b011)
    }
}
