use ::rand::{rng, rngs::ThreadRng};
use rand_distr::Uniform;

pub struct SurvivorRng {
        pub(crate)  rng: ThreadRng,
        pub(crate) x_pos_gen: Uniform<f32>,
        pub(crate) y_pos_gen: Uniform<f32>,
}

impl SurvivorRng {
    pub fn new(low_x: f32, high_x: f32, low_y: f32, high_y: f32) -> Self {
        let rng = rng();
        let x_pos_gen = 
        Uniform::new_inclusive(low_x, high_x).expect("Failed to create uniform distribution: invalid range");
        let y_pos_gen = 
        Uniform::new_inclusive(low_y, high_y).expect("Failed to create uniform distribution: invalid range");
        
        SurvivorRng {
            rng,
            x_pos_gen,
            y_pos_gen,
        }
    }
}