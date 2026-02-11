use macroquad::prelude::*;
use rand_distr::Distribution;

use crate::collision::{Collidable, hitbox_intersects};
use crate::entity::character::Direction;
use crate::entity::ennemy::Ennemy;
use crate::entity::player::Player;
use crate::survivor_rng::SurvivorRng;
use crate::weapons::dagger::DaggerAggregate;
use crate::weapons::sword::Sword;

const MOVE_DISTANCE: f32 = 1.;
const PLAYER_RADIUS: f32 = 10.;
const MAX_ENNEMIES_NB: u8 = 10;
const ENNEMY_SPEED: f32 = 0.1;

const MAP_WIDTH: f32 = 2048.;
const MAP_HEIGHT: f32 = 2048.;


pub struct Game {
    player: Player,
    ennemies: Vec<Ennemy>,
    score: i16,
    rng: SurvivorRng,
    player_idle_texture: Texture2D,
    player_walking_texture: Texture2D,
    orc_texture: Texture2D,
    grass_texture: Texture2D,
}

pub struct GameData {
    pub(crate) is_game_over: bool,
    pub(crate) score: i16,
}


impl Game {
    pub(crate) fn new(sword_texture: &Texture2D, player_idle_texture: &Texture2D, 
        player_walking_texture: &Texture2D, dagger_texture: &Texture2D, orc_texture: &Texture2D, grass_texture: &Texture2D) -> Self {
        let sword = Sword::new(
            vec2(screen_width() / 2.0, screen_height() / 2.),
            0.,
            0.2,
            sword_texture.clone()
        );
            
        let daggers = DaggerAggregate::new(dagger_texture.clone());
        
        let player = Player::new(
            Vec2::new(
                screen_width() / 2.0, 
                screen_height() / 2.0
            ), 
            sword,
            daggers,
        );
        
        
        let ennemies: Vec<Ennemy> = Vec::new();
        
        let score: i16 = 0;
        
        let rng = SurvivorRng::new(
            PLAYER_RADIUS, 
            screen_width() - PLAYER_RADIUS, 
            PLAYER_RADIUS, 
            screen_height() - PLAYER_RADIUS
        );
        
        Game {
            player,
            ennemies,
            score,
            rng,
            player_idle_texture: player_idle_texture.clone(),
            player_walking_texture: player_walking_texture.clone(),
            orc_texture: orc_texture.clone(),
            grass_texture: grass_texture.clone(),
        }
    }
        
    pub(crate) fn update(&mut self) -> GameData {
        self.get_input();
        self.player.udpate();
        self.manage_collisions();
        self.populate_ennemies();
        self.draw();
        
        GameData {
            is_game_over: self.player.character.hp <= 0,
            score: self.score,
        }
    }
        
    fn manage_collisions(&mut self) {
        // Moving ennemies + checking ennemies - player collision
        for ennemy in self.ennemies.iter_mut() {
            let direction = get_direction_from_vector(ennemy.vel);
            ennemy.move_by(ennemy.vel * ENNEMY_SPEED, direction);
            
            if hitbox_intersects(&ennemy.hitbox(), &self.player.hitbox()) {
                self.player.character.hp -= 1;
                ennemy.collided = true;
            }
            if self.player.weapons_collides_with(&ennemy.hitbox()) {
                self.score += 1;
                ennemy.collided = true;
            }
        }
        
        self.ennemies.retain(|ennemy| !ennemy.collided);
    }
        
    fn get_input(&mut self) {
        let mut player_movement: Vec2;
        let player_direction: Direction;
        
        if is_key_down(KeyCode::Down) {
            player_movement = Vec2::new(0., MOVE_DISTANCE);
            player_direction = Direction::Down;
        } else if is_key_down(KeyCode::Up) {
            player_movement = Vec2::new(0., -MOVE_DISTANCE);
            player_direction = Direction::Up;
        } else if is_key_down(KeyCode::Right) {
            player_movement = Vec2::new(MOVE_DISTANCE, 0.);
            player_direction = Direction::Right;
        } else if is_key_down(KeyCode::Left) {
            player_movement = Vec2::new(-MOVE_DISTANCE, 0.);
            player_direction = Direction::Left;
        } else {
            player_movement = Vec2::new(0., 0.);
            player_direction = Direction::None;
        }
        
        // Prevent player from moving outside of the map
        if self.player.character.world_position.x + player_movement.x < 0. {
            player_movement.x = -self.player.character.world_position.x;
        } else if self.player.character.world_position.x + player_movement.x > MAP_WIDTH {
            player_movement.x = MAP_WIDTH - self.player.character.world_position.x;
        }
        if self.player.character.world_position.y + player_movement.y < 0. {
            player_movement.y = -self.player.character.world_position.y;
        } else if self.player.character.world_position.y + player_movement.y > MAP_HEIGHT {
            player_movement.y = MAP_HEIGHT - self.player.character.world_position.y;
        }

        self.player.move_by(player_movement, player_direction);

        adjust_ennemies_velocity(&mut self.ennemies, &self.player);
        
        
        if is_key_pressed(KeyCode::Space) {
            let mut mouse_pos = Vec2::new(0., 0.);
            (mouse_pos.x, mouse_pos.y) = mouse_position();

            println!("Mouse position: {:?}", mouse_pos);
            
            let normalize_vect = compute_normalized_vector(
                Vec2{x: screen_width() / 2., y: screen_height() / 2.}, mouse_pos);
                
            self.player.throw_dagger(normalize_vect, normalize_vect.y.atan2(normalize_vect.x));
        }
    }
            
    fn draw(&mut self) {
        // Screen origin (upper left corner) in world coordinates
        let screen_origin_position = Vec2{
            x: self.player.character.world_position.x - screen_width() / 2.,
            y: self.player.character.world_position.y - screen_height() / 2.,
        };

        let screen_rect = Rect::new(
            screen_origin_position.x, 
            screen_origin_position.y, 
            screen_width(), 
            screen_height()
        );

        draw_texture_ex(&self.grass_texture, 0., 0., WHITE, DrawTextureParams {
            source: Some(screen_rect),
            ..Default::default()
        });

        for ennemy in self.ennemies.iter_mut() {
            ennemy.draw(screen_origin_position, &self.orc_texture);
        }
        self.player.draw(screen_origin_position, &self.player_idle_texture, &self.player_walking_texture);
        draw_text(&format!("Score : {}", self.score), 10., 15., 20., WHITE);
        draw_text(&format!("HP : {}", self.player.character.hp), 10., 32., 20., WHITE);
    }
            
    fn populate_ennemies(&mut self) {
        while self.ennemies.len() < MAX_ENNEMIES_NB.into() {
            let new_ennemy_pos = Vec2 { 
                x: self.rng.x_pos_gen.sample(&mut self.rng.rng), 
                y: self.rng.y_pos_gen.sample(&mut self.rng.rng) 
            };
            self.ennemies.push(Ennemy::new( 
                new_ennemy_pos, 
                compute_normalized_vector(new_ennemy_pos, self.player.character.world_position),
            ));
        }
    }
}
        
fn adjust_ennemies_velocity(ennemies: &mut Vec<Ennemy>, player: &Player) {
    for ennemy in ennemies.iter_mut() {
        ennemy.vel = compute_normalized_vector(ennemy.character.world_position, player.character.world_position);
    }
}
        
fn compute_normalized_vector(pos_start: Vec2, pos_end: Vec2) -> Vec2 {
    let vector = pos_end - pos_start;
    
    let y_member = pos_end.y - pos_start.y;
    let x_member = pos_end.x - pos_start.x;
    
    let magnitude = vector.length();
    
    Vec2::new(x_member / magnitude, y_member / magnitude)
}
        
fn get_direction_from_vector(vector: Vec2) -> Direction {
    if vector.x == 0. && vector.y == 0. {
        Direction::None
    } else if vector.x.abs() > vector.y.abs() {
        if vector.x > 0. {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        if vector.y > 0. {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}
        
        