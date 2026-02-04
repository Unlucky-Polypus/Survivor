use macroquad::prelude::*;

mod draw_utils {
    fn draw_rect(&self, local_pos: Vec2, size: Vec2, color: Color) {
        draw_rectangle_ex(
            self.position.x + local_pos.x,
            self.position.y + local_pos.y,
            size.x,
            size.y,
            DrawRectangleParams {
                rotation: self.angle,
                offset: vec2(0.0, 0.0),
                color,
                ..Default::default()
            },
        );
    }
}