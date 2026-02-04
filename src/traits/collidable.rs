use crate::collision::Hitbox;

pub trait Collidable {
    fn hitbox(&self) -> Hitbox;
}