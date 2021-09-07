use bevy::prelude::*;

use crate::screen::*;

pub struct ClearQuads(pub Vec<bool>);

pub const QUAD_SIZE: f32 = 8.;

impl ClearQuads {
    pub fn new() -> Self {
        ClearQuads(vec![
            false;
            (SCREEN_WIDTH as usize / QUAD_SIZE as usize)
                * (SCREEN_HEIGHT as usize / QUAD_SIZE as usize)
        ])
    }
}

pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearQuads::new());
    }
}
