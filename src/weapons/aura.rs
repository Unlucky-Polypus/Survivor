use macroquad::prelude::*;

use crate::{collision::{Collidable, Hitbox}};

const AURA_COLOR: Color = DARKPURPLE;
const AURA_OPACITY: f32 = 0.5;

pub(crate) struct Aura {
    pub(crate) circle: Circle,
}

impl Aura{
    /// Draw the sword taking into account its rotation and position
    pub fn draw(&self, screen_center_position: Vec2) {
        let screen_position = Vec2 {
            x: self.circle.x - screen_center_position.x,
            y: self.circle.y - screen_center_position.y,
        };
        draw_circle(screen_position.x, screen_position.y, self.circle.r, AURA_COLOR.with_alpha(AURA_OPACITY));
    }
}

impl Collidable for Aura {
    fn hitbox(&self) -> Hitbox {
        Hitbox::Circle(self.circle)
    }
}