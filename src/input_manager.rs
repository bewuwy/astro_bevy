use bevy::prelude::*;

pub fn get_mouse_world_pos(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    if let Some(mouse_pos) = window.cursor_position() {
        // get mouse position in world space
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (mouse_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    } else {
        Vec2::ZERO
    }
}
