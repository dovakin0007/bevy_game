pub mod components;
pub mod resources;
pub mod system;

use bevy::prelude::*;

use resources::*;
use system::*;

pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const NO_OF_STARS: usize = 10;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App){
        app.init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_stars)
        .add_system(start_star_spawn_timer)
        .add_system(spawn_stars_over_time);
    }
}

