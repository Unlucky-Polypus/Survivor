use macroquad::prelude::*;
use crate::{collision::{Collidable, Hitbox, HitboxParams}, weapons::weapon::{OBBWeapon, WeaponHitboxParams}};

// The sword hitbox is 60% of the png size from the tip of the sword to the handle
const HITBOX_WIDTH_RATIO: f32 = 0.7;
const HITBOX_HEIGTH_RATIO: f32 = 0.53; // Blade width = 115 pixels, png height is 215 px
const SWORD_WIDTH: f32 = 897.;
const SWORD_HEIGHT: f32 = 216.;


pub struct Sword {
    pub(crate) weapon: OBBWeapon,
    texture: Texture2D,
}

impl Sword {
    pub fn new(position: Vec2, angle: f32, size_ratio: f32, texture: Texture2D) -> Self {
        let hitbox_params = WeaponHitboxParams {
            params: HitboxParams {
                size: Vec2 { x: SWORD_WIDTH, y: SWORD_HEIGHT},
                offset_frame: Vec2 { x: -20., y: 0. },
            },
            width_ratio: HITBOX_WIDTH_RATIO,
            height_ratio: HITBOX_HEIGTH_RATIO,
        };
        Self {
            weapon: OBBWeapon::new(position, angle, size_ratio, hitbox_params),
            texture,
        }
    }
    
    pub fn update(&mut self) {
        // For example, we can make the sword rotate over time
        let dt = get_frame_time();
        self.weapon.angle += 2.0 * dt; // Rotate at 2 radians per second
    }

    /// Draw the sword taking into account its rotation and position
    pub fn draw(&self, screen_center_position: Vec2) {
        self.weapon.draw(&self.texture, screen_center_position, Vec2 { 
            x: 20.0, 
            y: -(self.texture.size().y * self.weapon.size_ratio / 2.0) 
        });
    }
}

/// Implement Collidable trait for Sword to provide its hitbox
impl Collidable for Sword {
    fn hitbox(&self) -> Hitbox {
        self.weapon.hitbox()
    }
}