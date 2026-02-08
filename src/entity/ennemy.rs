use macroquad::prelude::*;

use crate::{collision::Hitbox, entity::character::{CharTextureParams, Character, Direction}, traits::collidable::Collidable};

const FRAME_DURATION: f32 = 0.12; // Duration of each animation frame in seconds
const NB_FRAMES: u8 = 11; // Number of frames in the player animation
const FRAME_WIDTH: f32 = 64.0; // Width of each frame in the sprite sheet
const FRAME_HEIGHT: f32 = 64.0; // Height of each frame in the sprite sheet
const ENNEMY_WIDTH: f32 = 29.0; // Width of the ennemy hitbox
const ENNEMY_HEIGHT: f32 = 41.0; // Height of the enn

pub(crate) struct Ennemy {
    pub(crate) character: Character,
    pub(crate) vel: Vec2,
    pub(crate) collided: bool,
}

impl Ennemy {
    pub(crate) fn new(pos: Vec2, vel: Vec2) -> Self {
        Ennemy {
            character: Character::new(pos, Vec2 { x: ENNEMY_WIDTH, y: ENNEMY_HEIGHT }),
            vel,
            collided: false,
        }
    }
    
    pub(crate) fn move_by(&mut self, movement: Vec2, direction: Direction) {
        self.character.move_by(movement, direction);
    }
    
    pub(crate) fn draw(&mut self, texture: &Texture2D) {
        self.character.draw(texture, texture, &CharTextureParams {
            frame_duration: FRAME_DURATION,
            nb_frames: NB_FRAMES,
            frame_width: FRAME_WIDTH,
            frame_height: FRAME_HEIGHT,
        });
    }
}

impl Collidable for Ennemy {
    fn hitbox(&self) -> Hitbox {
        self.character.hitbox()
    }
}