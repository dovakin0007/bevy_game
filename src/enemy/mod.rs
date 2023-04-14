pub mod components;
pub mod resources;
pub mod system;

use bevy::prelude::*;

use resources::*;
use system::*;

pub const NO_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED:f32 = 200.0;
pub const ENEMY_SIZE:f32 = 64.0; // this is the enemy sprite size
pub const ENEMY_SPAWN_TIME: f32 = 7.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App){
        app.init_resource::<EnemySpawnTimer>()
            .add_startup_system(spawn_enemy)
            .add_system(enemy_movement)
            .add_system(confine_enemy_movement)
            .add_system(update_enemy_movement)
            .add_system(start_enemy_spawn_timer)
            .add_system(start_enemy_spawn_time);
    }
}