use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::RenderLayers;

pub struct DebugPlugin;
const RENDER_LAYER: u8 = 1;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_state(DebugState::Disabled)
            .add_system_set(
                SystemSet::on_enter(DebugState::Disabled).with_system(hide_debug.system()),
            )
            .add_system_set(
                SystemSet::on_enter(DebugState::Enabled).with_system(show_debug.system()),
            )
            .add_system_set(
                SystemSet::on_update(DebugState::Enabled).with_system(update_frame_data.system()),
            )
            .add_system(debug_state_toggle.system());
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DebugState {
    Enabled,
    Disabled,
}

pub struct FPSText;
pub struct UiCamera;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(RenderLayers::all())
        .insert(UiCamera);

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "\nFrame time: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 15.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(RenderLayers::layer(RENDER_LAYER))
        .insert(FPSText);
}

pub fn hide_debug(_commands: Commands, query: Query<&mut RenderLayers, With<Camera>>) {
    query.for_each_mut(|mut r| {
        *r = r.without(RENDER_LAYER);
    });
}

pub fn show_debug(_commands: Commands, query: Query<&mut RenderLayers, With<Camera>>) {
    query.for_each_mut(|mut r| {
        *r = r.with(RENDER_LAYER);
    });
}

pub fn update_frame_data(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.sections[1].value = format!("{:.1}", fps);

        text.sections[3].value = format!("{:.3}", frame_time * 1000.0);
    }
}

fn debug_state_toggle(
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_state: ResMut<State<DebugState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        let curr = *debug_state.current();
        debug_state
            .set(if curr == DebugState::Disabled {
                DebugState::Enabled
            } else {
                DebugState::Disabled
            })
            .unwrap();
    }
}
