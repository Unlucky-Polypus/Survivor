use macroquad::prelude::*;

use crate::{collision::Hitbox, collision::Collidable};

const RADIUS: f32 = 10.;

pub(crate) struct Bullet {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) collided: bool,
}

impl Collidable for Bullet {
    fn hitbox(&self) -> Hitbox {
        Hitbox::Circle(Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: RADIUS,
        })
    }
}