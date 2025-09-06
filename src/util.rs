use crate::{BOARD_SQUARE_SIZE, BOARD_SQUARES};
use eframe::egui::{Pos2, Rect, Vec2};

pub fn board_size_vec2() -> Vec2 {
    Vec2::splat((BOARD_SQUARES * BOARD_SQUARE_SIZE) as f32)
}

pub fn viewport_size_vec2() -> Vec2 {
    let board_size = (BOARD_SQUARES * BOARD_SQUARE_SIZE) as f32;
    Vec2::new(board_size + 100.0, board_size + 50.0)
}

pub fn promotion_size_vec2() -> Vec2 {
    Vec2::new(BOARD_SQUARE_SIZE as f32, (BOARD_SQUARE_SIZE * 4) as f32)
}

pub fn promotion_selection_rect_pos() -> Pos2 {
    Pos2 {
        x: board_size_vec2().x + 25.0,
        y: 25.0,
    }
}

pub fn promotion_selection_rect() -> Rect {
    Rect::from_min_size(promotion_selection_rect_pos(), promotion_size_vec2())
}
