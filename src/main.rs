use std::{panic};

use macroquad::prelude::*;

use crate::game::Game;

mod sword;
mod traits;
mod collision;
mod game;
mod entity;
mod survivor_rng;

enum GameState {
    Game,
    Pause,
    GameOver,
    MainMenu,
}


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
    
    let player_idle_texture_result = load_texture("assets/proto1_idle.png").await;
    let player_idle_texture: Texture2D;
    match player_idle_texture_result {
        Ok(texture) => {
            player_idle_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }
    
    let player_walking_texture_result = load_texture("assets/proto1_walk.png").await;
    let player_walking_texture: Texture2D;
    match player_walking_texture_result {
        Ok(texture) => {
            player_walking_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }
    
    set_default_filter_mode(FilterMode::Nearest);
    
    let mut game = Game::new(&sword_texture, &player_idle_texture, &player_walking_texture);
    let mut game_state = GameState::MainMenu;
    
    loop {
        clear_background(BLACK);
        match game_state {
            GameState::MainMenu => {
                game_state = state_main_menu();
            }
            GameState::Game => {
                game_state = state_game(&mut game);
            }
            GameState::Pause => {
                game_state = state_pause();
            }
            GameState::GameOver => {
                draw_text("Game Over! Press any key to restart.", 10., 10., 20., WHITE);
                if !get_keys_pressed().is_empty() {
                    game = Game::new(&sword_texture, &player_idle_texture, &player_walking_texture);
                    game_state = GameState::Game;
                }
            }
        }
        draw_fps();
        next_frame().await;
    }
}

fn state_game(game: &mut Game) -> GameState {
    if is_key_pressed(KeyCode::Escape) {
        return GameState::Pause;
    }
    let game_data = game.update();
    if game_data.is_game_over {
        GameState::GameOver
    } else {
        GameState::Game
    }
    // println!("Game data: is_game_over = {}, score = {}", game_data.is_game_over, game_data.score);
    // if game_data.is_game_over {
    //     break;
    // }
    
    // let last_frame_time = get_frame_time();
    // if last_frame_time < 1.0 {
    //     println!("Sleeping for {} seconds", 1.0 - last_frame_time);
    //     sleep(std::time::Duration::from_secs_f32(1.0 - last_frame_time));
    // }
}

fn state_main_menu() -> GameState {
    draw_text("Press any key to start", 10., 10., 20., WHITE);
    if !get_keys_pressed().is_empty() {
        println!("Starting the game...");
        GameState::Game
    } else {
        GameState::MainMenu
    }
}

fn state_pause() -> GameState {
    draw_text("Game paused. Press any key to resume.", 10., 10., 20., WHITE);
    if !get_keys_pressed().is_empty() {
        GameState::Game
    } else {
        GameState::Pause
    }
    
}

