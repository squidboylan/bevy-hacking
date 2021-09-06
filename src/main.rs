use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod debug;

struct Player;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(60.0, 60.0)),
        ..Default::default()
    })
}

fn main() {
    App::build()
        /*
        .insert_resource(WindowDescriptor {
            vsync: false,
            ..Default::default()
        })
        */
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup.system())
        .add_state(debug::DebugState::Disabled)
        .add_startup_system(debug::setup.system())
        .add_system_set(
            SystemSet::on_enter(debug::DebugState::Disabled)
                .with_system(debug::hide_debug.system()),
        )
        .add_system_set(
            SystemSet::on_enter(debug::DebugState::Enabled).with_system(debug::show_debug.system()),
        )
        .add_system_set(
            SystemSet::on_update(debug::DebugState::Enabled)
                .with_system(debug::update_frame_data.system()),
        )
        .add_system(keyboard_input_system.system())
        .run();
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_state: ResMut<State<debug::DebugState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        let curr = *debug_state.current();
        debug_state
            .set(if curr == debug::DebugState::Disabled {
                debug::DebugState::Enabled
            } else {
                debug::DebugState::Disabled
            })
            .unwrap();
    }
}
