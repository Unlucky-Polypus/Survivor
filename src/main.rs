use macroquad::prelude::*;
use ::rand::{rng, rngs::ThreadRng};
use rand_distr::{Distribution, Uniform};


const MOVE_DISTANCE: f32 = 2.;
const BULLET_RADIUS: f32 = 3.;
const BULLET_SPEED: f32 = 4.;
const PLAYER_RADIUS: f32 = 10.;
const MAX_ENNEMIES_NB: u8 = 10;
const ENNEMY_SPEED: f32 = 0.1;

struct Player {
    pos: Vec2,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    collided: bool,
}

struct Ennemy {
    pos: Vec2,
    vel: Vec2,
    collided: bool,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut player = Player {
        pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0) 
    };

    let mut bullets: Vec<Bullet> = Vec::new();

    let mut ennemies: Vec<Ennemy> = Vec::new();

    let mut score: i16 = 0;

    let mut player_hp: i16 = 10;

    let mut rng: ThreadRng = rng();
    let x_pos_gen = 
Uniform::new_inclusive(PLAYER_RADIUS, screen_width() - PLAYER_RADIUS).expect("Failed to create uniform distribution: invalid range");
    let y_pos_gen = 
Uniform::new_inclusive(PLAYER_RADIUS, screen_height() - PLAYER_RADIUS).expect("Failed to create uniform distribution: invalid range");

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Down) {
            player.pos.y += MOVE_DISTANCE;
            adjust_ennemies_velocity(&mut ennemies, &player);
        }

        if is_key_down(KeyCode::Up) {
            player.pos.y -= MOVE_DISTANCE;
            adjust_ennemies_velocity(&mut ennemies, &player);
        }

        if is_key_down(KeyCode::Right) {
            player.pos.x += MOVE_DISTANCE;
            adjust_ennemies_velocity(&mut ennemies, &player);
        }

        if is_key_down(KeyCode::Left) {
            player.pos.x -= MOVE_DISTANCE;
            adjust_ennemies_velocity(&mut ennemies, &player);
        }

        if is_key_pressed(KeyCode::Space) {
            let mut mouse_pos = Vec2::new(0., 0.);
            (mouse_pos.x, mouse_pos.y) = mouse_position();
            
            let normalize_vect = compute_normalized_vector(player.pos, mouse_pos);

            bullets.push(Bullet { pos: player.pos, vel: normalize_vect, collided: false });
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Moving bullets + checking bullet - ennemies collisions
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel * BULLET_SPEED;
            
            for ennemy in ennemies.iter_mut() {
                if (ennemy.pos - bullet.pos).length() < PLAYER_RADIUS {
                    score += 1;
                    bullet.collided = true;
                    ennemy.collided = true;
                } 
            }
        }

        // Moving ennemies + checking ennemies - player collision
        for ennemy in ennemies.iter_mut() {
            if (ennemy.pos - player.pos).length() < PLAYER_RADIUS {
                ennemy.collided = true;
                player_hp -= 1;
            } else {
                    ennemy.pos += ennemy.vel * ENNEMY_SPEED;
                }
        }

        bullets.retain(|bullet| !bullet.collided);
        ennemies.retain(|ennemy| !ennemy.collided);
        populate_ennemies(&mut ennemies, &player, x_pos_gen, y_pos_gen, &mut rng);

        draw_player(&player);
        draw_ennemies(&ennemies);
        draw_bullets(&bullets);
        draw_text(&format!("Score : {score}"), 10., 15., 20., WHITE);
        draw_text(&format!("HP : {player_hp}"), 10., 32., 20., WHITE);
        next_frame().await
    }
}

fn draw_player(player: &Player) {
    draw_circle(player.pos.x, player.pos.y, PLAYER_RADIUS, BLUE);
}

fn draw_bullets(bullets: &Vec<Bullet>) {
    for bullet in bullets {
        draw_circle(bullet.pos.x, bullet.pos.y, BULLET_RADIUS, WHITE);
    }
}

fn draw_ennemies(ennemies: &Vec<Ennemy>) {
    for ennemy in ennemies {
        draw_circle(ennemy.pos.x, ennemy.pos.y, PLAYER_RADIUS, RED);
    }
}

fn compute_normalized_vector(pos_start: Vec2, pos_end: Vec2) -> Vec2 {
    let vector = pos_end - pos_start;

    let y_member = pos_end.y - pos_start.y;
    let x_member = pos_end.x - pos_start.x;

    let magnitude = vector.length();

    Vec2::new(x_member / magnitude, y_member / magnitude)
}

fn populate_ennemies(
    ennemies: &mut Vec<Ennemy>, 
    player: &Player,
    x_pos_gen: Uniform<f32>, 
    y_pos_gen: Uniform<f32>, 
    rng: &mut ThreadRng) {
        while ennemies.len() < MAX_ENNEMIES_NB.into() {
        let new_ennemy_pos = Vec2 { x: x_pos_gen.sample(rng), y: y_pos_gen.sample(rng) };
        ennemies.push(Ennemy { 
            pos: new_ennemy_pos, 
            vel: compute_normalized_vector(new_ennemy_pos, player.pos),
            collided: false
        });
    }
}

fn adjust_ennemies_velocity(
    ennemies: &mut Vec<Ennemy>, 
    player: &Player) {
        for ennemy in ennemies.iter_mut() {
            ennemy.vel = compute_normalized_vector(ennemy.pos, player.pos);
        }
    }

