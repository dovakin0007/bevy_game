use rand::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// use crate::enemy::components::*;
// use crate::enemy::resources::*;
use super::components::*;
use super::resources::*;
use super::{NO_OF_ENEMIES, ENEMY_SPEED, ENEMY_SIZE};


pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
 ){
    let window =  window_query.get_single().unwrap();
 
    for _i in 0..NO_OF_ENEMIES{
        let rand_x = (random::<f32>() * window.width()) / 1.2;
        let rand_y = (random::<f32>() * window.height()) / 1.2;
 
        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                }, Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );
    }
}


pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform , &Enemy)>,
    time: Res<Time>
 ){
    for (mut transform, enemy ) in enemy_query.iter_mut(){
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds(); 
    } 
 }
 pub fn update_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>, 
    asset_server: Res<AssetServer>,
 ){
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let window = window_query.get_single().unwrap();
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
 
    for (transform, mut enemy ) in enemy_query.iter_mut(){
        let translation = transform.translation;
        
        let mut direction_change = false;
        if translation.x < x_min || translation.x > x_max{
            enemy.direction *= -1.0;
            direction_change = true;
        } 
        if translation.y < y_min || translation.y > y_max{
            enemy.direction *= -1.0;
            direction_change = true;
        }
 
        if direction_change {
            let sound_effect_1 = asset_server.load("audio/tick_001.ogg");
            let sound_effect_2 = asset_server.load("audio/tick_001.ogg");
 
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            }else{
                sound_effect_2
            };
 
            audio.play(sound_effect);
        }
 
    }
 }
 pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
 )
 {
   let window = window_query.get_single().unwrap();
   let half_enemy_size = ENEMY_SIZE / 2.0;
   let x_max = window.width() - half_enemy_size;
   let y_max = window.height() - half_enemy_size;
   let x_min = 0.0 + half_enemy_size;
   let y_min = 0.0 + half_enemy_size;
   
   for transform in enemy_query.iter_mut(){
        let mut transalation = transform.translation;
 
        if transalation.x < x_min{
            transalation.x =  x_min;
 
        }else if transalation.x > x_max {
            transalation.x = x_max;
        }
 
        if transalation.y < y_min{
            transalation.y = y_min;
        }
        else if transalation.y > y_max {
            transalation.y = y_max;
        }
   }
 }
 
 
pub fn start_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
 ){
    enemy_spawn_timer.timer.tick(time.delta());
 }
 
 pub fn start_enemy_spawn_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
 ){
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
       let random_x = random::<f32>() * window.width();
       let random_y = random::<f32>() * window.height();
 
       commands.spawn((SpriteBundle{
           transform: Transform::from_xyz(random_x, random_y, 0.0),
           texture: asset_server.load("sprites/ball_red_large.png"),
           ..default()
       }, Enemy{
           direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
       },));
 }
 }
 