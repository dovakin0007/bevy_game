use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::{PLAYER_SPEED, PLAYER_SIZE, ENEMY_SIZE, STAR_SIZE};
use crate::enemy::components::*;
use crate::events::GameOver;
use crate::score::resources::*;
use crate::star::components::*;


pub fn spawn_player(
    mut commands: Commands,
   window_query: Query<&Window, With<PrimaryWindow>>, 
   asset_server: Res<AssetServer>){
       let window = window_query.get_single().unwrap();

       commands.spawn((SpriteBundle{
           transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
           texture: asset_server.load("sprites/ball_blue_large.png"),
           ..default()

       }, Player {},));
}



pub fn player_movments(
   keyboard_input: Res<Input<KeyCode>>,
   mut player_query: Query<& mut Transform, With<Player>>,
   time: Res<Time>,
){
   let mut direction = Vec3::ZERO;

   if let Ok(mut transform) = player_query.get_single_mut(){
       if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
           direction += Vec3::new(-1.0, 0.0, 0.0);
       }
       if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
           direction += Vec3::new(1.0, 0.0, 0.0);
       }
       if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
           direction += Vec3::new(0.0, 1.0, 0.0);
       }
       if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
           direction += Vec3::new(0.0, -1.0, 0.0);
       }
       
       if direction.length()> 0.0 {
           direction = direction.normalize();
       }

       transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

   }
}
pub fn confine_player_movement(
   mut player_query:Query<&mut Transform, With<Player>>,
   window_query:Query<&Window, With<PrimaryWindow>>
){
   if let Ok(mut player_transform) = player_query.get_single_mut() {
       let window = window_query.get_single().unwrap();

       let half_player_size = PLAYER_SIZE / 2.0; 
       let x_min = 0.0 + half_player_size;
       let x_max = window.width() - half_player_size;
       let y_min = 0.0 + half_player_size;
       let y_max = window.height() - half_player_size;

       let mut transalation = player_transform.translation;
       
       // to bind with the window
       if transalation.x < x_min {
           transalation.x = x_min;
       }
       else if transalation.x > x_max {
           transalation.x = x_max;
       }

       if transalation.y < y_min {
           transalation.y = y_min;
       }else if transalation.y > y_max {
           transalation.y = y_max;
       }

       player_transform.translation = transalation;
   }
}



pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>,
 ){
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut(){
        for enemy_transform in enemy_query.iter() {
            let distance =  player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius =  ENEMY_SIZE / 2.0;
            if distance < enemy_radius + player_radius{
                println!(" Game over!");
                let sound_effect = asset_server.load("audio/impactMetal_light_003.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver {score: score.val});
            };
 
        }
    }
 }

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity ,&Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
 ){
    if let Ok(player_transform) = player_query.get_single(){
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            
            if distance < player_radius + star_radius {
                score.val += 1; 
                let sound_effect = asset_server.load("audio/impactMetal_light_003.ogg");
                audio.play(sound_effect);
                println!("You scored a point");
 
                commands.entity(star_entity).despawn();
            }
 
        }
    }
 }
 
 
