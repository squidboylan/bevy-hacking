use bevy::prelude::*;
use bitflags::bitflags;

use crate::screen::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Player;

bitflags! {
    pub struct PathQuad: u8 {
        const OCCUPIED = 0b00000001;
        const CHANGED = 0b00000010;
        const STILL_OCCUPIED = 0b00000100;
    }
}

pub struct PathGrid(pub Vec<PathQuad>);

pub const QUAD_SIZE: f32 = 4.;

impl PathGrid {
    pub fn new() -> Self {
        let c = PathGrid(vec![
            PathQuad::empty();
            (SCREEN_WIDTH as usize / QUAD_SIZE as usize)
                * (SCREEN_HEIGHT as usize / QUAD_SIZE as usize)
        ]);
        c
    }
}

pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathGrid::new())
            .add_system(update_collision_grid);
    }
}

fn update_collision_grid(
    mut collision_grid: ResMut<PathGrid>,
    query: Query<(&Sprite, &Transform), With<Player>>,
) {
    for q in collision_grid.0.iter_mut() {
        q.remove(PathQuad::STILL_OCCUPIED);
    }
    for (s, t) in query.iter() {
        let p_x = t.translation.x - s.custom_size.unwrap().x / 2. + SCREEN_WIDTH / 2.;
        let p_y = t.translation.y - s.custom_size.unwrap().y / 2. + SCREEN_HEIGHT / 2.;
        let p_w = s.custom_size.unwrap().x;
        let p_h = s.custom_size.unwrap().y;

        let quad_x = p_x as usize / QUAD_SIZE as usize;
        let quad_y = p_y as usize / QUAD_SIZE as usize;
        for x in quad_x..=(p_x + p_w - 0.1) as usize / QUAD_SIZE as usize {
            for y in quad_y..=(p_y + p_h - 0.1) as usize / QUAD_SIZE as usize {
                let i = y * SCREEN_WIDTH as usize / QUAD_SIZE as usize + x;
                if !collision_grid.0[i].contains(PathQuad::OCCUPIED) {
                    collision_grid.0[i] = PathQuad::all();
                } else {
                    collision_grid.0[i].insert(PathQuad::STILL_OCCUPIED);
                }
            }
        }
    }
    for q in collision_grid.0.iter_mut() {
        if !q.contains(PathQuad::STILL_OCCUPIED) {
            *q = PathQuad::empty();
        }
    }
}
