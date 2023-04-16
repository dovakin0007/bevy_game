// mod components;
// mod events;
// mod resources;

use crate::AppState;
use crate::game::events::*;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
 ){
    let window = window_query.get_single().unwrap();
 
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            ..default()
        }
    );
 }

pub fn transition_to_game_state(
   mut commands: Commands,
   keyboard_input: Res<Input<KeyCode>>,
   app_state: Res<State<AppState>>
){
   if keyboard_input.just_pressed(KeyCode::G){
      if app_state.0 != AppState::Game{
         commands.insert_resource(NextState(Some(AppState::Game)));
         println!("Switched to Game State");
      }
   }
}

pub fn transition_to_main_menu_state(
   mut commands: Commands,
   keyboard_input: Res<Input<KeyCode>>,
   app_state: Res<State<AppState>>,
){
   if keyboard_input.just_pressed(KeyCode::M){
      if app_state.0 != AppState::Menu{
         commands.insert_resource(NextState(Some(AppState::Menu)));
         println!("Switched to Menu State");
      }
   }
}

pub fn exit_game(
   keyboard_input: Res<Input<KeyCode>>,
   mut app_exit_event_writer: EventWriter<AppExit>
){
   if keyboard_input.just_pressed(KeyCode::Escape) {
       app_exit_event_writer.send(AppExit);
   }
}

pub fn handle_game_over(
   mut event_game_over_reader: EventReader<GameOver>
){
   for event in event_game_over_reader.iter(){
       println!("Final Score is {}", event.score.to_string());
   }
}
