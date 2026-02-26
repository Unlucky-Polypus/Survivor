use std::{panic};

use macroquad::prelude::*;

use crate::game::Game;
use crate::survivor_ui::window::Action;
use crate::survivor_ui::main_menu_window::MainMenuWindow;
use crate::survivor_ui::pause_window::PauseWindow;

mod weapons;
mod collision;
mod game;
mod entity;
mod survivor_rng;
mod draw_utils;
mod survivor_ui;

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
    
    let player_idle_texture_result = load_texture("assets/player_idle.png").await;
    let player_idle_texture: Texture2D;
    match player_idle_texture_result {
        Ok(texture) => {
            player_idle_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }
    
    let player_walking_texture_result = load_texture("assets/player_walk.png").await;
    let player_walking_texture: Texture2D;
    match player_walking_texture_result {
        Ok(texture) => {
            player_walking_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }

    let dagger_texture_result = load_texture("assets/dagger.png").await;
    let dagger_texture: Texture2D;
    match dagger_texture_result {
        Ok(texture) => {
            dagger_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }

    let orc_texture_result = load_texture("assets/orc.png").await;
    let orc_texture: Texture2D;
    match orc_texture_result {
        Ok(texture) => {
            orc_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }

    let grass_texture_result = load_texture("assets/grass_zoom.png").await;
    let grass_texture: Texture2D;
    match grass_texture_result {
        Ok(texture) => {
            grass_texture = texture;
        }
        Err(error) => panic!("{error}"),
    }

    let main_menu_window = MainMenuWindow::new().await;
    let pause_window = PauseWindow::new().await;

    set_default_filter_mode(FilterMode::Nearest);
    
    let mut game = Game::new(&sword_texture, &player_idle_texture, &player_walking_texture, 
        &dagger_texture, &orc_texture, &grass_texture);
    let mut game_state = GameState::MainMenu;

    println!("Screen width: {}, Screen height: {}", screen_width(), screen_height());
    
    loop {
        match game_state {
            GameState::MainMenu => {
                game_state = state_main_menu(&main_menu_window);
            }
            GameState::Game => {
                game_state = state_game(&mut game);
            }
            GameState::Pause => {
                game_state = state_pause(&pause_window);
            }
            GameState::GameOver => {
                draw_text("Game Over! Press any key to restart.", 10., 10., 20., WHITE);
                if !get_keys_pressed().is_empty() {
                    game = Game::new(&sword_texture, &player_idle_texture, &player_walking_texture,
                        &dagger_texture, &orc_texture, &grass_texture);
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

fn state_main_menu(main_menu_window: &MainMenuWindow) -> GameState {
    let action = main_menu_window.draw();
    if let Some(action) = action {
        match action {
            Action::Play => GameState::Game,
            Action::Options => GameState::MainMenu, // In a real app, this would go to options screen
            Action::Quit => GameState::GameOver,
            _ => GameState::MainMenu, // This case should not happen, but we handle it just in case
        }
    } else {
        GameState::MainMenu
    }
}

fn state_pause(pause_window: &PauseWindow) -> GameState {
    let action = pause_window.draw();
    if let Some(action) = action {
        match action {
            Action::Resume => GameState::Game,
            Action::Options => GameState::MainMenu, // In a real app, this would go to options screen
            Action::QuitToMainMenu => GameState::MainMenu,
            _ => GameState::Pause, // This case should not happen, but we handle it just in case
        }
    } else {
        GameState::Pause
    }
    
}

