use macroquad::prelude::*;

use crate::{collision::Hitbox, sword::Sword, traits::collidable::Collidable};

const RADIUS: f32 = 10.;

pub(crate) struct Player {
   pub(crate) pos: Vec2,
    pub(crate) hp: i16,
    sword: Sword,
}

impl Player {
    pub(crate) fn new(pos: Vec2, sword: Sword) -> Self {
        Player {
            pos,
            hp: 1,
            sword,
        }
    }

    pub(crate) fn udpate(&mut self) {
        let dt = get_frame_time();
        self.sword.angle += 2.0 * dt;
    }

    pub(crate) fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, RADIUS, BLUE);
        self.sword.draw();
    }

    pub(crate) fn weapon_hitbox(&self) -> Hitbox {
        self.sword.hitbox()
    }

    pub(crate) fn move_by(&mut self, direction: Vec2) {
        self.pos += direction;
        self.sword.position = self.pos;
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