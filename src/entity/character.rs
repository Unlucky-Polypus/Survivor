use macroquad::prelude::*;

use crate::{collision::{Collidable, Hitbox, HitboxParams, OBB}, draw_utils::is_on_screen};

pub(crate) struct Character {
    pub(crate) world_position: Vec2,
    pub(crate) hp: i16,
    direction: Direction,
    hitbox_params: HitboxParams,
    anim_timer: f32,
    frame: u8,
    is_idle: bool,
}

impl Character {
    pub(crate) fn new(pos: Vec2, hitbox_params: HitboxParams) -> Self {
        Character {
            world_position: pos,
            hp: 1,
            direction: Direction::Down,
            hitbox_params,
            anim_timer: 0.0,
            frame: 0,
            is_idle: true,
        }
    }
    
    pub(crate) fn draw(&mut self, idle_texture: &Texture2D, walking_texture: &Texture2D, 
        params: &CharTextureParams, screen_origin_position: Vec2) {
        let screen_position = Vec2 {
            x: self.world_position.x - screen_origin_position.x,
            y: self.world_position.y - screen_origin_position.y,
        };

        // Don't draw the character if it's not on screen
        if !is_on_screen(screen_position) {
            return;
        }

        let texture = if self.is_idle { idle_texture } else { walking_texture };
        
        let row = match self.direction {
            Direction::None => 0,
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        };
        
        let source: Rect = Rect::new(
            self.frame as f32 * params.frame_width,
            row as f32 * params.frame_height,
            params.frame_width,
            params.frame_height,
        );
        
        if self.anim_timer >= params.frame_duration {
            self.anim_timer = 0.0;
            self.frame = (self.frame + 1) % params.nb_frames;
        }
        
        draw_texture_ex(
            texture,
            screen_position.x - params.frame_width / 2.0,
            screen_position.y - params.frame_height / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(Vec2::new(params.frame_width, params.frame_height)),
                ..Default::default()
            },
        );
        
        // Debug: draw the sword hitbox in debug builds
        #[cfg(debug_assertions)]
        {
            use crate::collision;

            collision::draw_hitbox(&self.hitbox(), screen_origin_position, RED);
        }
    }
    
    pub(crate) fn move_by(&mut self, movement: Vec2, direction: Direction) {
        match direction {
            Direction::None => {
                self.anim_timer = 0.0;
                self.frame = 0;
                self.is_idle = true;
            }
            _ => {
                self.is_idle = false;
                self.anim_timer += get_frame_time();
                self.world_position += movement;
                self.direction = direction;
            }
        }
    }
}

impl Collidable for Character {
    fn hitbox(&self) -> Hitbox {
        Hitbox::OBB(OBB {
            world_center_position: Vec2 { 
                x: self.world_position.x + self.hitbox_params.offset_frame.x,
                y: self.world_position.y + self.hitbox_params.offset_frame.y,
            },
            half: Vec2 {
                x: self.hitbox_params.size.x / 2.0,
                y: self.hitbox_params.size.y / 2.0,
            },
            rotation: 0.0,
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

pub struct CharTextureParams {
    pub(crate) frame_width: f32,
    pub(crate) frame_height: f32,
    pub(crate) nb_frames: u8,
    pub(crate) frame_duration: f32,
}