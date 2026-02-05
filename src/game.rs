use macroquad::prelude::*;
use rand_distr::Distribution;

use crate::entity::bullet::Bullet;
use crate::entity::ennemy::Ennemy;
use crate::entity::player::Player;
use crate::survivor_rng::SurvivorRng;
use crate::sword::Sword;
use crate::traits::collidable::Collidable;

const MOVE_DISTANCE: f32 = 2.;
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

impl Game {
    pub(crate) fn new(sword_texture: Texture2D) -> Self {
        let sword = Sword {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            angle: 0.0,
            texture: sword_texture,
            size_ratio: 8.0,
        };

        let player = Player {
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            hp: 10,
            sword: sword
        };
        
        
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
    
    pub(crate) fn update(&mut self) {
        self.get_input();
        self.player.udpate();
        self.manage_collisions();
        self.populate_ennemies();
        self.draw();
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
        for ennemy in self.ennemies.iter_mut() {
            ennemy.pos += ennemy.vel * ENNEMY_SPEED;
            if crate::collision::hitbox_intersects(&weapon_hitbox, &ennemy.hitbox()) {
                self.score += 1;
                ennemy.collided = true;
            }
        }

        self.bullets.retain(|bullet| !bullet.collided);
        self.ennemies.retain(|ennemy| !ennemy.collided);
    }

    fn get_input(&mut self) {
        if is_key_down(KeyCode::Down) {
            self.player.pos.y += MOVE_DISTANCE;
            Self::adjust_ennemies_velocity(&mut self.ennemies, &self.player);
        }
        
        if is_key_down(KeyCode::Up) {
            self.player.pos.y -= MOVE_DISTANCE;
            Self::adjust_ennemies_velocity(&mut self.ennemies, &self.player);
        }
        
        if is_key_down(KeyCode::Right) {
            self.player.pos.x += MOVE_DISTANCE;
            Self::adjust_ennemies_velocity(&mut self.ennemies, &self.player);
        }
        
        if is_key_down(KeyCode::Left) {
            self.player.pos.x -= MOVE_DISTANCE;
            Self::adjust_ennemies_velocity(&mut self.ennemies, &self.player);
        }
        
        if is_key_pressed(KeyCode::Space) {
            let mut mouse_pos = Vec2::new(0., 0.);
            (mouse_pos.x, mouse_pos.y) = mouse_position();
            
            let normalize_vect = compute_normalized_vector(self.player.pos, mouse_pos);
            
            self.bullets.push(Bullet { pos: self.player.pos, vel: normalize_vect, collided: false });
        }
    }
    
    fn draw(&self) {
        clear_background(BLACK);
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
        
        fn adjust_ennemies_velocity(ennemies: &mut Vec<Ennemy>, player: &Player) {
            for ennemy in ennemies.iter_mut() {
                ennemy.vel = compute_normalized_vector(ennemy.pos, player.pos);
            }
        }
        
    }

    fn compute_normalized_vector(pos_start: Vec2, pos_end: Vec2) -> Vec2 {
        let vector = pos_end - pos_start;
        
        let y_member = pos_end.y - pos_start.y;
        let x_member = pos_end.x - pos_start.x;
        
        let magnitude = vector.length();
        
        Vec2::new(x_member / magnitude, y_member / magnitude)
    }