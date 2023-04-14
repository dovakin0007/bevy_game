
pub mod events;
pub mod system;

mod enemy;
mod player;
mod score;
mod star;

use events::*;
use system::*;

use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;


use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<GameOver>()
    .add_plugin(EnemyPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(ScorePlugin)
    .add_plugin(StarPlugin)
    .add_startup_system(spawn_camera)
    .add_system(handle_game_over)
    .add_system(exit_game)
    .run();
}

