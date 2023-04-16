pub mod components;
pub mod resources;
pub mod system;

use bevy::prelude::*;

use crate::AppState;

use resources::*;
use system::*;

use super::SimulationState;

pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const NO_OF_STARS: usize = 10;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App){
        app.init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_stars)
        .add_systems((spawn_stars_over_time, 
            start_star_spawn_timer)
            .in_set(OnUpdate(AppState::Game)).in_set(OnUpdate(SimulationState::Running)));
        // .add_system(start_star_spawn_timer)
        // .add_system(spawn_stars_over_time);
    }
}

