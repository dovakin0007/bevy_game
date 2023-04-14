pub mod components;
pub mod system;

use bevy::prelude::*;

use system::*;

pub const  PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE:f32 = 64.0;
pub const ENEMY_SIZE:f32 = 64.0;
pub const STAR_SIZE: f32 = 30.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app.add_startup_system(spawn_player)
        .add_system(player_movments)
        .add_system(confine_player_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star);
    }
}