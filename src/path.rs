use bevy::prelude::*;
use bitflags::bitflags;

use crate::screen::*;

pub struct Player;

bitflags! {
    pub struct PathQuad: u8 {
        const OCCUPIED = 0b00000001;
        const CHANGED = 0b00000010;
    }
}

pub struct PathGrid(pub Vec<PathQuad>);

pub const QUAD_SIZE: f32 = 8.;

impl PathGrid {
    pub fn new() -> Self {
        let mut c = PathGrid(vec![
            PathQuad::empty();
            (SCREEN_WIDTH as usize / QUAD_SIZE as usize)
                * (SCREEN_HEIGHT as usize / QUAD_SIZE as usize)
        ]);
        /*
        c.0[0] = PathQuad::all();
        c.0[10] = PathQuad::all();
        c.0[20] = PathQuad::all();
        c.0[30] = PathQuad::all();
        c.0[40] = PathQuad::all();
        */
        c
    }
}

pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathGrid::new())
            .add_system(update_collision_grid.system());
    }
}

/// TODO: Update to use 2 vecs that we flip to represent last frame and new frame so the debug
/// system can be appropriately "notified" of when grid spaces are not longer occupied
fn update_collision_grid(
    mut collision_grid: ResMut<PathGrid>,
    query: Query<(&Sprite, &Transform), With<Player>>,
) {
    for (s, t) in query.iter() {
        let p_x = t.translation.x - s.size.x / 2. + SCREEN_WIDTH / 2.;
        let p_y = t.translation.y - s.size.y / 2. + SCREEN_HEIGHT / 2.;
        let p_w = s.size.x;
        let p_h = s.size.y;

        let quad_x = p_x as usize / QUAD_SIZE as usize;
        let quad_y = p_y as usize / QUAD_SIZE as usize;
        for x in quad_x..quad_x + p_w as usize / QUAD_SIZE as usize {
            for y in quad_y..quad_y + p_h as usize / QUAD_SIZE as usize {
                let i = y * SCREEN_WIDTH as usize / QUAD_SIZE as usize + x;
                if !collision_grid.0[i].contains(PathQuad::OCCUPIED) {
                    collision_grid.0[i] = PathQuad::all();
                }
            }
        }
    }
}
