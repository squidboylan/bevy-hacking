use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::RenderLayers;
use bevy_prototype_lyon::prelude::*;

use crate::path::PathGrid;
use crate::path::PathQuad;
use crate::path::QUAD_SIZE;
use crate::screen::*;

pub struct DebugPlugin;
const RENDER_LAYER: u8 = 1;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system())
            .add_plugin(ShapePlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_state(DebugState::Disabled)
            .add_system(update_debug_collision_grid.system())
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

    let line = shapes::Line(
        Vec2::new(0., -SCREEN_HEIGHT / 2.),
        Vec2::new(0., SCREEN_HEIGHT / 2.),
    );
    let mut x = -SCREEN_WIDTH / 2.;

    while x <= SCREEN_WIDTH / 2. {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                ShapeColors::new(Color::rgba(0., 0., 0., 0.5)),
                DrawMode::Stroke(StrokeOptions::default().with_line_width(1.)),
                Transform::from_xyz(x, 0., 1.),
            ))
            .insert(RenderLayers::layer(RENDER_LAYER));
        x += QUAD_SIZE;
    }
    let line = shapes::Line(
        Vec2::new(-SCREEN_WIDTH / 2., 0.),
        Vec2::new(SCREEN_WIDTH / 2., 0.),
    );
    let mut y = -SCREEN_HEIGHT / 2.;
    while y <= SCREEN_HEIGHT / 2. {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                ShapeColors::new(Color::rgba(0., 0., 0., 0.5)),
                DrawMode::Stroke(StrokeOptions::default().with_line_width(1.)),
                Transform::from_xyz(0., y, 1.),
            ))
            .insert(RenderLayers::layer(RENDER_LAYER));
        y += QUAD_SIZE;
    }
}

pub fn hide_debug(_commands: Commands, mut query: Query<&mut RenderLayers, With<Camera>>) {
    query.for_each_mut(|mut r| {
        *r = r.without(RENDER_LAYER);
    });
}

pub fn show_debug(_commands: Commands, mut query: Query<&mut RenderLayers, With<Camera>>) {
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

struct PathQuadIndex(usize);

fn update_debug_collision_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut collision_grid: ResMut<PathGrid>,
    mut query: Query<(Entity, &PathQuadIndex)>,
) {
    for (e, i) in query.iter_mut() {
        if !collision_grid.0[i.0].contains(PathQuad::OCCUPIED) {
            commands.entity(e).despawn();
        }
    }
    for (i, c) in collision_grid.0.iter_mut().enumerate() {
        if c.contains(PathQuad::CHANGED | PathQuad::OCCUPIED) {
            let x = i * QUAD_SIZE as usize % SCREEN_WIDTH as usize;
            let y = (i * QUAD_SIZE as usize / SCREEN_WIDTH as usize) * QUAD_SIZE as usize;
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.5).into()),
                    transform: Transform::from_xyz(
                        (x as f32) + QUAD_SIZE / 2. - SCREEN_WIDTH / 2.,
                        (y as f32) + QUAD_SIZE / 2. - SCREEN_HEIGHT / 2.,
                        1.0,
                    ),
                    sprite: Sprite::new(Vec2::new(QUAD_SIZE, QUAD_SIZE)),
                    ..Default::default()
                })
                .insert(PathQuadIndex(i))
                .insert(RenderLayers::layer(RENDER_LAYER));

            c.remove(PathQuad::CHANGED);
        }
    }
}
