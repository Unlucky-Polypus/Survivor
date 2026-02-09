use macroquad::prelude::*;

use crate::collision::{self, Collidable, Hitbox, HitboxParams, OBB};

pub struct Weapon {
    pub position: Vec2,
    pub angle: f32,
    pub size_ratio: f32,
    hitbox_params: WeaponHitboxParams,
}

// pub trait WeaponTrait {
//     fn update(&self);
// }

impl Weapon {
    pub fn new(position: Vec2, angle: f32, size_ratio: f32, 
        hitbox_params: WeaponHitboxParams) -> Self {
        Self {
            position,
            angle,
            size_ratio,
            hitbox_params,
        }
    }
    /// Draw the weapon taking into account its rotation and position
    pub fn draw(&self, texture: &Texture2D, offset: Vec2) {
        
        // The pivot point is at the center of the handle and the position is at the top-left 
        // corner of the texture
        let pivot = Vec2{
            x: self.position.x,
            y: self.position.y
        };

        let texture_position = self.position + offset;
        
        draw_texture_ex(
            texture,
            texture_position.x,
            texture_position.y,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                pivot: Some(pivot),
                dest_size: Some(self.adjusted_size()),
                ..Default::default()
            },
        );
        
        // Debug: draw the sword hitbox in debug builds
        #[cfg(debug_assertions)]
        {
            collision::draw_hitbox(&self.hitbox(), RED);
        }
    }

    /// Scale the size of the texture depending on the size ratio
    fn adjusted_size(&self) -> Vec2 {
        Vec2 { 
            x: self.hitbox_params.params.size.x * self.size_ratio, 
            y: self.hitbox_params.params.size.y * self.size_ratio 
        }
    }
}

/// Implement Collidable trait for Weapon to provide its hitbox
impl Collidable for Weapon {
    fn hitbox(&self) -> Hitbox {
        let adjusted_size = self.adjusted_size();
        let height = self.hitbox_params.height_ratio * adjusted_size.y;
        // `self.position` is the top-left corner of the drawn texture,
        // so compute hitbox position directly from it.
        let x = self.position.x + ((1.0 - self.hitbox_params.width_ratio) * adjusted_size.x - self.hitbox_params.params.offset_frame.x);
        let y = self.position.y + ((1.0 - self.hitbox_params.height_ratio) * adjusted_size.y) - (adjusted_size.y - (adjusted_size.y - height) / 2.0) - self.hitbox_params.params.offset_frame.y;
        // Width and height of the hitbox (the blade)
        let w = self.hitbox_params.width_ratio * adjusted_size.x;
        let h = height;

        let half_size = Vec2{ x: w / 2.0, y: h / 2.0 };
        // Center of the hitbox in world coordinates must account for rotation
        // around the pivot (`self.position` is the top-left corner / draw pivot).
        let local_center = Vec2 { x: x + half_size.x, y: y + half_size.y };
        let rot = Mat2::from_angle(self.angle);
        let center = self.position + rot * (local_center - self.position);

        collision::Hitbox::OBB(OBB {
            center,
            half: half_size,
            rotation: self.angle,
        })

    }
}

pub struct WeaponHitboxParams {
    pub params: HitboxParams,
    pub width_ratio: f32,
    pub height_ratio: f32,
}