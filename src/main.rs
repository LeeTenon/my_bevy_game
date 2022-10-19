use bevy::prelude::*;
fn main() {
    App::new()
        .add_startup_system(setip_system)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setip_system(mut c: Commands, a: Res<AssetServer>) {
    // camera
    c.spawn_bundle(Camera2dBundle::default());
    c.spawn_bundle(SpriteBundle {
        texture: a.load("../assets/plane.png"),
        transform: Transform {
            translation: Vec3::new(0., 100., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

// #[derive(Component)]
// struct Person;
