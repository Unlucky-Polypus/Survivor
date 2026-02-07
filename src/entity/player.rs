use std::u8;

use macroquad::prelude::*;

use crate::{collision::Hitbox, sword::Sword, traits::collidable::Collidable};

const RADIUS: f32 = 10.;
const FRAME_DURATION: f32 = 0.12; // Duration of each animation frame in seconds
const NB_FRAMES: u8 = 8; // Number of frames in the player animation
const FRAME_WIDTH: f32 = 192.0; // Width of each frame in the sprite sheet
const FRAME_HEIGHT: f32 = 192.0; // Height of each frame in the sprite sheet

pub(crate) struct Player {
    pub(crate) pos: Vec2,
    pub(crate) hp: i16,
    sword: Sword,
    direction: Direction,
    anim_timer: f32,
    frame: u8,
    idle_texture: Texture2D,
    walking_texture: Texture2D,
    is_idle: bool,
}

impl Player {
    pub(crate) fn new(pos: Vec2, sword: Sword, idle_texture: Texture2D, walking_texture: Texture2D) -> Self {
        Player {
            pos,
            hp: 1,
            sword,
            direction: Direction::Down,
            anim_timer: 0.0,
            frame: 0,
            idle_texture,
            walking_texture,
            is_idle: true,
        }
    }
    
    pub(crate) fn udpate(&mut self) {
        let dt = get_frame_time();
        self.sword.angle += 2.0 * dt;
    }
    
    pub(crate) fn draw(&self) {
        let texture = if self.is_idle { 
            &self.idle_texture 
        } else { 
            &self.walking_texture 
        };

        let row = match self.direction {
            Direction::None => 0,
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        };
        
        let source: Rect = Rect::new(
            self.frame as f32 * FRAME_WIDTH,
            row as f32 * FRAME_HEIGHT,
            FRAME_WIDTH,
            FRAME_HEIGHT,
        );

        draw_texture_ex(
            texture,
            self.pos.x - FRAME_WIDTH / 2.0,
            self.pos.y - FRAME_HEIGHT / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(Vec2::new(FRAME_WIDTH, FRAME_HEIGHT)),
                ..Default::default()
            },
        );
        self.sword.draw();
    }

    pub(crate) fn weapon_hitbox(&self) -> Hitbox {
        self.sword.hitbox()
    }
    
    pub(crate) fn move_by(&mut self, movement: Vec2, player_direction: Direction) {
        match player_direction {
            Direction::None => {
                self.anim_timer = 0.0;
                self.frame = 0;
                self.is_idle = true;
            }
            _ => {
                self.is_idle = false;
                self.anim_timer += get_frame_time();
                if self.anim_timer >= FRAME_DURATION {
                    self.anim_timer = 0.0;
                    self.frame = (self.frame + 1) % NB_FRAMES;
                }
                self.pos += movement;
                self.direction = player_direction;
                self.sword.position = self.pos;
            }
        }
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Hitbox {
        Hitbox::Circle(Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: RADIUS,
        })
    }
}

pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}