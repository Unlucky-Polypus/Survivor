use macroquad::prelude::*;

use crate::{collision::Hitbox, sword::Sword, traits::collidable::Collidable};

const RADIUS: f32 = 10.;

pub(crate) struct Player {
    pub(crate) pos: Vec2,
    pub(crate) hp: i16,
    pub(crate) sword: Sword,
}

impl Player {
    pub fn udpate(&mut self) {
        let dt = get_frame_time();
        self.sword.angle += 2.0 * dt;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, RADIUS, BLUE);
        self.sword.draw();
    }

    pub(crate) fn weapon_hitbox(&self) -> Hitbox {
        self.sword.hitbox()
    }
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