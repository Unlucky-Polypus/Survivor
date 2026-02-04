use macroquad::prelude::*;
use crate::{collision, collision::{Hitbox, OBB}, traits::collidable::Collidable};

// The sword hitbox is 60% of the png size from the tip of the sword to the handle
const HITBOX_WIDTH_RATIO: f32 = 0.7;
const HITBOX_HEIGTH_RATIO: f32 = 0.53; // Blade width = 115 pixels, png height is 215 px


pub struct Sword {
    pub position: Vec2,
    pub angle: f32,
    pub texture: Texture2D,
    pub size_ratio: f32,
}

impl Sword {
    /// Draw the sword taking into account its rotation and position
    pub fn draw(&self) {
        
        // The pivot point is at the center of the handle and the position is at the top-left 
        // corner of the texture
        let pivot = Vec2{
            x: self.position.x,
            y: self.position.y
        };

        let texture_position = Vec2 {
            x: self.position.x + 12.0,
            y: self.position.y - (self.texture.size().y / self.size_ratio / 2.0)
        };

        // draw_circle(texture_position.x, texture_position.y, 5.0, RED);
        // draw_circle(pivot.x, pivot.y, 5.0, GREEN); 
        // draw_circle(self.position.x, self.position.y, 5.0, BLUE);
        
        draw_texture_ex(
            &self.texture,
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
            x: &self.texture.size().x / self.size_ratio, 
            y: &self.texture.size().y / self.size_ratio 
        }
    }
}

/// Implement Collidable trait for Sword to provide its hitbox
impl Collidable for Sword {
    fn hitbox(&self) -> Hitbox {
        let adjusted_size = self.adjusted_size();
        let height = HITBOX_HEIGTH_RATIO * adjusted_size.y;
        // `self.position` is the top-left corner of the drawn texture,
        // so compute hitbox position directly from it.
        let x = self.position.x + ((1.0 - HITBOX_WIDTH_RATIO) * adjusted_size.x) + 12.0;
        let y = self.position.y + ((1.0 - HITBOX_HEIGTH_RATIO) * adjusted_size.y) - (adjusted_size.y - (adjusted_size.y - height) / 2.0);
        // Width and height of the hitbox (the blade)
        let w = HITBOX_WIDTH_RATIO * adjusted_size.x;
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