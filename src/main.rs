use bevy::prelude::*;
use bevy::render::camera::RenderLayers;

mod debug;
mod path;
mod screen;
use crate::screen::*;

struct Player;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(RenderLayers::all());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .insert(Player);
    /*
    let shape = shapes::Line((-1000.0, -100.0).into(), (1000.0, 100.0).into());

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::new(Color::BLACK),
        DrawMode::Stroke(StrokeOptions::default().with_line_width(2.)),
        //Transform::default(),
        Transform::from_xyz(0.0, 100., 0.0),
    ));
    */
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            //vsync: false,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            scale_factor_override: Some(2.0),
            ..Default::default()
        })
        //.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(path::PathPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_startup_system(setup.system())
        .run();
}
