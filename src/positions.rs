use eframe::egui::Pos2;

use crate::{BOARD_SQUARE_SIZE, BOARD_SQUARES};

pub fn turn_info_text_position() -> Pos2 {
    let board_size = (BOARD_SQUARES * BOARD_SQUARE_SIZE) as f32;
    Pos2::new(25.0, board_size + 25.0)
}
