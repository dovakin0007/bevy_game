use bevy::prelude::*;

use crate::events::*;
use super::resources::*;

pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score : {}", score.val.to_string());
    }
 }
 
pub fn handle_game_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>, 
 ){
    for event in game_over_event_reader.iter(){
        high_score.scores.push(("Player".to_string(), event.score));
    }
 }
 
 
 pub fn high_score_is_updated(high_score: Res<HighScore>){
    if high_score.is_changed(){
        println!("High scores is {:?}", high_score);
    }
 }
 


