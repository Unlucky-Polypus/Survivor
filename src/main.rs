use macroquad::prelude::*;

const MOVE_DISTANCE: f32 = 2.;
const BULLET_RADIUS: f32 = 3.;
const PLAYER_RADIUS: f32 = 10.;

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

    ennemies.push(Ennemy { 
        pos: Vec2::new(screen_width() / 4., screen_height() / 4.),
        collided: false 
    });

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Down) {
            player.pos.y += MOVE_DISTANCE;
        }

        if is_key_down(KeyCode::Up) {
            player.pos.y -= MOVE_DISTANCE;
        }

        if is_key_down(KeyCode::Right) {
            player.pos.x += MOVE_DISTANCE;
        }

        if is_key_down(KeyCode::Left) {
            player.pos.x -= MOVE_DISTANCE;
        }

        if is_key_pressed(KeyCode::Space) {
            let mut mouse_pos = Vec2::new(0., 0.);
            (mouse_pos.x, mouse_pos.y) = mouse_position();
            
            let player_cursor_vec = mouse_pos - player.pos;
            let y_member = mouse_pos.y - player.pos.y;
            let x_member = mouse_pos.x - player.pos.x;

            let distance = player_cursor_vec.length();

            let normalize_vect = Vec2::new(x_member / distance, y_member / distance);

            bullets.push(Bullet { pos: player.pos, vel: normalize_vect, collided: false });
            println!("Shot !");
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel;
            
            for ennemy in ennemies.iter_mut() {
                if (ennemy.pos - bullet.pos).length() < PLAYER_RADIUS {
                    score += 1;
                    bullet.collided = true;
                    ennemy.collided = true;
                }
            }
        }
        bullets.retain(|bullet| !bullet.collided);
        ennemies.retain(|ennemy| !ennemy.collided);

        draw_player(&player);
        draw_ennemies(&ennemies);
        draw_bullets(&bullets);
        draw_text(&format!("Score : {score}"), 10., 15., 20., WHITE);
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

