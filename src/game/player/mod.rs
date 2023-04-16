pub mod components;
pub mod system;

use bevy::prelude::*;

use system::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE:f32 = 64.0;
pub const ENEMY_SIZE:f32 = 64.0;
pub const STAR_SIZE: f32 = 30.0;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, PartialEq, Eq, Hash, Clone)]
pub struct ConfineSystemSet;




pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .configure_set(MovementSystemSet.before(ConfineSystemSet))
        .add_startup_system(spawn_player)
        // .add_system(player_movments.before(confine_player_movement))
        // .add_system(confine_player_movement)
        // .add_systems((
        //     player_movments.before(confine_player_movement),
        //     confine_player_movement
        // // )).chain()
        // .add_system(player_movments.in_set(MovementSystemSet))
        // .add_system(confine_player_movement.in_set(ConfineSystemSet))
        // .add_system(enemy_hit_player)
        // .add_system(player_hit_star)
        .add_systems((player_hit_star.in_set(MovementSystemSet),
                    player_movments.before(confine_player_movement),
                    confine_player_movement,
                    enemy_hit_player, 
                    player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)));
    }
}