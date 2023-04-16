pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod events;
mod system;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use events::*;
use system::*;
use crate::AppState;


pub struct GamePlugin ;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App){
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        //Game Over event custom event
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
        //User created Plugin which has systems in it

    }
}

#[derive(Debug, States, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default] 
    Paused,
}
