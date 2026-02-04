use macroquad::prelude::*;

use crate::{collision::Hitbox, traits::collidable::Collidable};

const PLAYER_RADIUS: f32 = 10.;

pub(crate) struct Ennemy {
    pub(crate) pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) collided: bool,
}

impl Collidable for Ennemy {
    fn hitbox(&self) -> Hitbox {
        Hitbox::Circle(Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: PLAYER_RADIUS,
        })
    }
}