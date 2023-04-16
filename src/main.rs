

pub mod system;

// mod enemy;
// mod player;
// mod score;
// mod star;

mod game;
mod main_menu;


use system::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;


use bevy::prelude::*;




fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugin(GamePlugin)
    .add_plugin(MainMenuPlugin)
    .add_startup_system(spawn_camera)
    .add_system(transition_to_game_state)
    .add_system(transition_to_main_menu_state)
    
    .add_system(handle_game_over)
    .add_system(exit_game)
    
   
    .run()

}

#[derive(States, PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub enum AppState {
    #[default]
    Menu,
    Game, 
    GameOver,
}

