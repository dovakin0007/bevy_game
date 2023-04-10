use bevy::app::AppExit;
use rand::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;


pub const NO_OF_ENEMIES: usize = 4;
pub const  PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE:f32 = 64.0; // this is the player sprite size
pub const ENEMY_SPEED:f32 = 200.0;
pub const ENEMY_SIZE:f32 = 64.0; // this is the enemy sprite size
pub const NO_OF_STARS:usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 7.0;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .init_resource::<StarSpawnTimer>()
    .init_resource::<EnemySpawnTimer>()
    .init_resource::<HighScore>()
    .add_event::<GameOver>()
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_enemy)
    .add_startup_system(spawn_stars)
    .add_system(player_movments)
    .add_system(confine_player_movement)
    .add_system(enemy_movement)
    .add_system(confine_enemy_movement)
    .add_system(update_enemy_movement)
    .add_system(enemy_hit_player)
    .add_system(player_hit_star)
    .add_system(update_score)
    .add_system(start_star_spawn_timer)
    .add_system(spawn_stars_over_time)
    .add_system(start_enemy_spawn_timer)
    .add_system(start_enemy_spawn_time)
    .add_system(handle_game_over)
    .add_system(exit_game)
    .add_system(handle_game_high_score)
    .add_system(high_score_is_updated)
    .run();
}

#[derive(Component)]
pub struct Player{

}

#[derive(Component)]
pub struct Enemy{
    pub direction:Vec2,

}

#[derive(Component)]
pub struct Star{

}

#[derive(Resource)]
pub struct Score{
    pub val: u32,
}

impl Default for Score{
    fn default() -> Score {
        Score { val: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer{
    pub timer: Timer,
}

impl Default for StarSpawnTimer{ 
     fn default() -> StarSpawnTimer {
        StarSpawnTimer { 
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating)
        } 
    }
    
}

#[derive(Resource)]

pub struct EnemySpawnTimer{
    pub timer: Timer,
}

impl Default for EnemySpawnTimer{
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}

pub struct GameOver {
    pub score:u32,
}


#[derive(Resource, Debug)]
pub struct HighScore{
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScore{
    fn default() -> HighScore{
        HighScore { 
            scores: Vec::new()
         }
    }
}

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
fn confine_player_movement(
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

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window =  window_query.get_single().unwrap();

    for _ in 0..NO_OF_STARS{
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn(
            (SpriteBundle{
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
                },
                Star{

                },
            )
        );
    }
}

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
fn update_enemy_movement(
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
fn confine_enemy_movement(
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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut score: Res<Score>,
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

pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score : {}", score.val.to_string());
    }
}

pub fn start_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>, 
    time: Res<Time>
){
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
){
    if star_spawn_timer.timer.finished(){
        let window =  window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((SpriteBundle{
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star{}
            )
    );
    }
}
fn start_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
){
    enemy_spawn_timer.timer.tick(time.delta());
}

fn start_enemy_spawn_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
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
