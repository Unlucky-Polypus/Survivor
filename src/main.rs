use std::panic;

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
        game.update();

        next_frame().await
    }   
}

    