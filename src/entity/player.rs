use std::u8;

use macroquad::prelude::*;

use crate::{collision::{Collidable, Hitbox, HitboxParams, hitbox_intersects}, entity::character::{CharTextureParams, Character, Direction}, weapons::{aura::Aura, dagger::DaggerAggregate, sword::Sword}};

const FRAME_DURATION: f32 = 0.12; // Duration of each animation frame in seconds
const NB_FRAMES: u8 = 8; // Number of frames in the player animation
const FRAME_WIDTH: f32 = 192.0; // Width of each frame in the sprite sheet
const FRAME_HEIGHT: f32 = 192.0; // Height of each frame in the sprite sheet
const PLAYER_WIDTH: f32 = 27.0; // Width of the player hitbox
const PLAYER_HEIGHT: f32 = 48.0; // Height of the player hitbox

pub(crate) struct Player {
    pub(crate) character: Character,
    sword: Sword,
    daggers: DaggerAggregate,
    aura: Aura,
}

impl Player {
    pub(crate) fn new(pos: Vec2, sword: Sword, daggers: DaggerAggregate) -> Self {
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
            daggers,
            aura: Aura { circle: Circle::new(pos.x, pos.y, 100.0) },

        }
    }
    
    pub(crate) fn udpate(&mut self) {
        self.sword.update();
        self.daggers.update();
        self.aura.circle.x = self.character.world_position.x;
        self.aura.circle.y = self.character.world_position.y;
    }
    
    pub(crate) fn draw(&mut self, screen_origin_position: Vec2, idle_texture: &Texture2D, walking_texture: &Texture2D) {
        self.aura.draw(screen_origin_position);
        self.character.draw(idle_texture, walking_texture, &CharTextureParams {
            frame_duration: FRAME_DURATION,
            nb_frames: NB_FRAMES,
            frame_width: FRAME_WIDTH,
            frame_height: FRAME_HEIGHT,
        }, screen_origin_position);
        self.sword.draw(screen_origin_position);
        self.daggers.draw(screen_origin_position);
    }
    
    pub(crate) fn weapons_collides_with(&mut self, hitbox: &Hitbox) -> bool {
        let weapon_hitbox = self.sword.hitbox();
        
        hitbox_intersects(&weapon_hitbox, hitbox) || 
        self.daggers.collide_with(hitbox) || 
        hitbox_intersects(&self.aura.hitbox(), hitbox)
    }
    
    pub(crate) fn move_by(&mut self, movement: Vec2, player_direction: Direction) {
        self.character.move_by(movement, player_direction);
        self.sword.weapon.world_position = self.character.world_position;
    }

    pub(crate) fn throw_dagger(&mut self, vel: Vec2, angle: f32) {
        self.daggers.new_dagger(self.character.world_position, vel, angle, 0.07);
    }
}

impl Collidable for Player {
    fn hitbox(&self) -> Hitbox {
        self.character.hitbox()
    }
}
