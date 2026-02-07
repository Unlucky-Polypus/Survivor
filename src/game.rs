use macroquad::prelude::*;
use rand_distr::Distribution;

use crate::collision::hitbox_intersects;
use crate::entity::bullet::Bullet;
use crate::entity::ennemy::Ennemy;
use crate::entity::player::{Direction, Player};
use crate::survivor_rng::SurvivorRng;
use crate::sword::Sword;
use crate::traits::collidable::Collidable;

const MOVE_DISTANCE: f32 = 1.;
const BULLET_RADIUS: f32 = 3.;
const BULLET_SPEED: f32 = 4.;
const PLAYER_RADIUS: f32 = 10.;
const MAX_ENNEMIES_NB: u8 = 10;
const ENNEMY_SPEED: f32 = 0.1;

pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    ennemies: Vec<Ennemy>,
    score: i16,
    rng: SurvivorRng,
}

pub struct GameData {
    pub(crate) is_game_over: bool,
    pub(crate) score: i16,
}


impl Game {
    pub(crate) fn new(sword_texture: &Texture2D, player_idle_texture: &Texture2D, player_walking_texture: &Texture2D) -> Self {
        let sword = Sword {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            angle: 0.0,
            texture: sword_texture.clone(),
            size_ratio: 8.0,
        };
        
        let player = Player::new(
            Vec2::new(
                screen_width() / 2.0, 
                screen_height() / 2.0
            ), 
            sword,
            player_idle_texture.clone(),
            player_walking_texture.clone()
        );
        
        
        let bullets: Vec<Bullet> = Vec::new();
        
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
            bullets,
            ennemies,
            score,
            rng,
        }
    }
    
    pub(crate) fn update(&mut self) -> GameData {
        self.get_input();
        self.player.udpate();
        self.manage_collisions();
        self.populate_ennemies();
        self.draw();
        
        GameData {
            is_game_over: self.player.hp <= 0,
            score: self.score,
        }
    }
    
    fn manage_collisions(&mut self) {
        // Moving bullets + checking bullet - ennemies collisions
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel * BULLET_SPEED;
            
            for ennemy in self.ennemies.iter_mut() {
                if (ennemy.pos - bullet.pos).length() < PLAYER_RADIUS {
                    self.score += 1;
                    bullet.collided = true;
                    ennemy.collided = true;
                } 
            }
        }
        
        let weapon_hitbox = self.player.weapon_hitbox();
        
        // Moving ennemies + checking ennemies - player collision
        // TODO : Player collision
        for ennemy in self.ennemies.iter_mut() {
            ennemy.pos += ennemy.vel * ENNEMY_SPEED;
            if hitbox_intersects(&ennemy.hitbox(), &self.player.hitbox()) {
                self.player.hp -= 1;
                ennemy.collided = true;
            }
            if hitbox_intersects(&weapon_hitbox, &ennemy.hitbox()) {
                self.score += 1;
                ennemy.collided = true;
            }
        }
        
        self.bullets.retain(|bullet| !bullet.collided);
        self.ennemies.retain(|ennemy| !ennemy.collided);
    }
    
    fn get_input(&mut self) {
        let player_movement: Vec2;
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
        
        adjust_ennemies_velocity(&mut self.ennemies, &self.player);
        
        self.player.move_by(player_movement, player_direction);
        
        if is_key_pressed(KeyCode::Space) {
            let mut mouse_pos = Vec2::new(0., 0.);
            (mouse_pos.x, mouse_pos.y) = mouse_position();
            
            let normalize_vect = compute_normalized_vector(self.player.pos, mouse_pos);
            
            self.bullets.push(Bullet { pos: self.player.pos, vel: normalize_vect, collided: false });
        }
    }
    
    fn draw(&self) {
        for bullet in self.bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, BULLET_RADIUS, WHITE);
        }
        for ennemy in self.ennemies.iter() {
            draw_circle(ennemy.pos.x, ennemy.pos.y, PLAYER_RADIUS, RED);
        }
        self.player.draw();
        draw_text(&format!("Score : {}", self.score), 10., 15., 20., WHITE);
        draw_text(&format!("HP : {}", self.player.hp), 10., 32., 20., WHITE);
    }
    
    fn populate_ennemies(&mut self) {
        while self.ennemies.len() < MAX_ENNEMIES_NB.into() {
            let new_ennemy_pos = Vec2 { 
                x: self.rng.x_pos_gen.sample(&mut self.rng.rng), 
                y: self.rng.y_pos_gen.sample(&mut self.rng.rng) 
            };
            self.ennemies.push(Ennemy { 
                pos: new_ennemy_pos, 
                vel: compute_normalized_vector(new_ennemy_pos, self.player.pos),
                collided: false
            });
        }
    }
}

fn adjust_ennemies_velocity(ennemies: &mut Vec<Ennemy>, player: &Player) {
    for ennemy in ennemies.iter_mut() {
        ennemy.vel = compute_normalized_vector(ennemy.pos, player.pos);
    }
}

fn compute_normalized_vector(pos_start: Vec2, pos_end: Vec2) -> Vec2 {
    let vector = pos_end - pos_start;
    
    let y_member = pos_end.y - pos_start.y;
    let x_member = pos_end.x - pos_start.x;
    
    let magnitude = vector.length();
    
    Vec2::new(x_member / magnitude, y_member / magnitude)
}
