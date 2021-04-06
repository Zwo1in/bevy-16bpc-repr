use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle_8bpc = asset_server.load("chainball-exported-8bpc.png");
    let texture_handle_16bpc = asset_server.load("chainball-exported-16bpc.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            material: materials.add(texture_handle_8bpc.into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            material: materials.add(texture_handle_16bpc.into()),
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 0.0)),
            ..Default::default()
        });
}
