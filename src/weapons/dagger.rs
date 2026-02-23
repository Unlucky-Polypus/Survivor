use macroquad::prelude::*;
use crate::{collision::{Collidable, Hitbox, HitboxParams, hitbox_intersects}, weapons::weapon::{OBBWeapon, WeaponHitboxParams}};

// The dagger hitbox is 60% of the png size from the tip of the dagger to the handle
const HITBOX_WIDTH_RATIO: f32 = 0.51;
const HITBOX_HEIGTH_RATIO: f32 = 0.55; // Blade width = 115 pixels, png height is 215 px
const DAGGER_WIDTH: f32 = 951.;
const DAGGER_HEIGHT: f32 = 256.;

pub struct DaggerAggregate {
    daggers: Vec<Dagger>,
    texture: Texture2D,
}

struct Dagger {
    weapon: OBBWeapon,
    vel: Vec2,
}

impl DaggerAggregate {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            daggers: Vec::new(),
            texture,
        }
    }

    pub fn new_dagger(&mut self, position: Vec2, vel: Vec2, angle: f32, size_ratio: f32) {
        let weapon = OBBWeapon::new(position, angle, size_ratio, WeaponHitboxParams {
            params: HitboxParams {
                size: Vec2 { x: DAGGER_WIDTH, y: DAGGER_HEIGHT},
                offset_frame: Vec2 { x: 0., y: 0. },
            },
            width_ratio: HITBOX_WIDTH_RATIO,
            height_ratio: HITBOX_HEIGTH_RATIO,
        });
        self.daggers.push(Dagger { weapon, vel });
    }

    pub fn update(&mut self) {
        for dagger in &mut self.daggers {
            dagger.weapon.world_position += dagger.vel;
        }
    }
    
    pub fn collide_with(&mut self, hitbox: &Hitbox) -> bool {
        if let Some(index) = self.daggers.iter().position(
            |dagger| hitbox_intersects(hitbox, &dagger.weapon.hitbox())
        ) {
            self.daggers.swap_remove(index);
            return true;
        }
        false
    }

    pub(crate) fn draw(&self, screen_center_position: Vec2) {
        for dagger in &self.daggers {
            dagger.weapon.draw(&self.texture, screen_center_position, Vec2 { 
                x: 0., 
                y: -(self.texture.size().y * dagger.weapon.size_ratio / 2.0) 
            });
        }
    }
}

// impl WeaponTrait for Dagger {
//     fn update(&self) {
//         // Update the dagger's state if necessary (e.g., animation, cooldown)
//     }
// }