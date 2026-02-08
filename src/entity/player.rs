use std::u8;

use macroquad::prelude::*;

use crate::{collision::{Collidable, Hitbox, HitboxParams}, entity::character::{CharTextureParams, Character, Direction}, sword::Sword};

const FRAME_DURATION: f32 = 0.12; // Duration of each animation frame in seconds
const NB_FRAMES: u8 = 8; // Number of frames in the player animation
const FRAME_WIDTH: f32 = 192.0; // Width of each frame in the sprite sheet
const FRAME_HEIGHT: f32 = 192.0; // Height of each frame in the sprite sheet
const PLAYER_WIDTH: f32 = 27.0; // Width of the player hitbox
const PLAYER_HEIGHT: f32 = 48.0; // Height of the player hitbox

pub(crate) struct Player {
    pub(crate) character: Character,
    sword: Sword,
}

impl Player {
    pub(crate) fn new(pos: Vec2, sword: Sword) -> Self {
        let hitbox_params = HitboxParams {
            size: Vec2 { x: PLAYER_WIDTH, y: PLAYER_HEIGHT },
            offset_frame: Vec2 { x: 0.0, y: 6.0 },
        };

        let character = Character::new(
            pos, 
            hitbox_params
        );
        
        Player {
            character,
            sword,
        }
    }
    
    pub(crate) fn udpate(&mut self) {
        let dt = get_frame_time();
        self.sword.angle += 2.0 * dt;
    }
    
    pub(crate) fn draw(&mut self, idle_texture: &Texture2D, walking_texture: &Texture2D, ) {
        self.character.draw(idle_texture, walking_texture, &CharTextureParams {
            frame_duration: FRAME_DURATION,
            nb_frames: NB_FRAMES,
            frame_width: FRAME_WIDTH,
            frame_height: FRAME_HEIGHT,
        });
        self.sword.draw();
    }
    
    pub(crate) fn weapon_hitbox(&self) -> Hitbox {
        self.sword.hitbox()
    }
    
    pub(crate) fn move_by(&mut self, movement: Vec2, player_direction: Direction) {
        self.character.move_by(movement, player_direction);
        self.sword.position = self.character.pos;
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Hitbox {
        self.character.hitbox()
    }
}
