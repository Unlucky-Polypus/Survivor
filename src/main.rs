use std::{panic, thread::sleep};

use macroquad::prelude::*;

use crate::game::Game;

mod sword;
mod traits;
mod collision;
mod game;
mod entity;
mod survivor_rng;


#[macroquad::main("BasicShapes")]
async fn main() {
    
    let sword_texture_result = load_texture("assets/sword.png").await;
    let sword_texture: Texture2D;
    match sword_texture_result {
        Ok(texture) => {
            sword_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }
    
    set_default_filter_mode(FilterMode::Nearest);
    
    let mut game = Game::new(sword_texture);
    loop {     
        let game_data = game.update();
        // println!("Game data: is_game_over = {}, score = {}", game_data.is_game_over, game_data.score);
        // if game_data.is_game_over {
        //     break;
        // }

        // let last_frame_time = get_frame_time();
        // if last_frame_time < 1.0 {
        //     println!("Sleeping for {} seconds", 1.0 - last_frame_time);
        //     sleep(std::time::Duration::from_secs_f32(1.0 - last_frame_time));
        // }
        draw_fps();
        next_frame().await;
    }   
}

    