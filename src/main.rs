use bevy::prelude::*;
use bevy::render::view::RenderLayers;

mod debug;
mod path;
mod screen;
use crate::path::Player;
use crate::screen::*;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(RenderLayers::all());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            //vsync: false,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        //.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(path::PathPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_startup_system(setup)
        .add_system(player_movement)
        .run();
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let delta = time.delta_seconds();
    if keyboard_input.pressed(KeyCode::W) {
        for mut p in query.iter_mut() {
            p.translation += delta * Vec3::from((0.0, 100.0, 0.0));
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for mut p in query.iter_mut() {
            p.translation += delta * Vec3::from((0.0, -100.0, 0.0));
        }
    }
    if keyboard_input.pressed(KeyCode::A) {
        for mut p in query.iter_mut() {
            p.translation += delta * Vec3::from((-100.0, 0.0, 0.0));
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for mut p in query.iter_mut() {
            p.translation += delta * Vec3::from((100.0, 0.0, 0.0));
        }
    }
}
