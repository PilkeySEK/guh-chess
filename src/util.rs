use crate::{BOARD_SQUARE_SIZE, BOARD_SQUARES};
use eframe::egui::Vec2;

pub fn board_size_vec2() -> Vec2 {
    Vec2::splat((BOARD_SQUARES * BOARD_SQUARE_SIZE) as f32)
}
