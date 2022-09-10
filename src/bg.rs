use bevy::prelude::*;


fn bg_setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bg.png"),
        transform: Transform::from_scale(Vec3::new(1., 1., 0.0)),
        // .with_translation(Vec3::new(0.0, WINDOW_HEIGHT/2.0, 0.0)),
        ..Default::default()
    });
}

pub struct BgPlugin;

impl Plugin for BgPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(bg_setup);
    }
}
