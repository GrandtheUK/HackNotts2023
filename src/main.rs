use bevy::{prelude::*, render::camera};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("Sprite-0001.png"),
            transform: Transform::from_xyz(-100., -100., 0.),
            ..default()
        }
    );
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("Sprite-0001.png"),
            transform: Transform::from_xyz(100., 100., 0.),
            ..default()
        }
    );
}


fn main() {
    App::new()
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .run();
}
