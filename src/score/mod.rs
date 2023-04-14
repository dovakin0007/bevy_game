
pub mod resources;
pub mod system;

use bevy::prelude::*;

use resources::*;
use system::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin{
    fn build(&self, app:&mut App){
        app.init_resource::<Score>()
        .init_resource::<HighScore>()
        .add_system(update_score)
        .add_system(handle_game_high_score)
        .add_system(high_score_is_updated);
    }
}