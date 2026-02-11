use macroquad::prelude::*;

/// Check if a screen position is within the screen bounds, given the screen size
/// The position is expected to be relative to the center of the screen, with (0, 0) being
/// the upper left corner of the screen and (screen_width(), screen_height()) being the lower right corner
pub fn is_on_screen(pos: Vec2) -> bool {
    pos.x >= 0.0 && pos.x <= screen_width() && pos.y >= 0.0 && pos.y <= screen_height()
}   