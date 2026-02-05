use macroquad::prelude::*;

use crate::{collision::Hitbox, traits::collidable::Collidable};

const RADIUS: f32 = 10.;

pub(crate) struct Player {
    pub(crate) pos: Vec2,
    pub(crate) hp: i16,
}

impl Collidable for Player {
    fn hitbox(&self) -> Hitbox {
        Hitbox::Circle(Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: RADIUS,
        })
    }
}