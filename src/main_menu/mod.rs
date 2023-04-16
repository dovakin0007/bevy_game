use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app:&mut App){
        app.add_startup_system(print_main_menu);
    }
}

fn print_main_menu(){
    println!("this is the main menu");
}