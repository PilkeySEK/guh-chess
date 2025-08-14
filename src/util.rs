use crate::{BOARD_SQUARE_SIZE, BOARD_SQUARES};
use eframe::egui::Vec2;

pub fn board_size_vec2() -> Vec2 {
    Vec2::splat((BOARD_SQUARES * BOARD_SQUARE_SIZE) as f32)
}

pub fn viewport_size_vec2() -> Vec2 {
    let board_size = (BOARD_SQUARES * BOARD_SQUARE_SIZE) as f32;
    Vec2::new(board_size, board_size + 50.0)
}
