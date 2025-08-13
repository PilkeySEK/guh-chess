use eframe::egui::Pos2;

use crate::{config::GameConfig, ChessApp};

pub fn screen_pos_to_board_index(app: &ChessApp, screen_pos: Pos2) -> u16 {
    let new_pos: (u16, u16) = (
        (screen_pos.x / app.config.board_square_size as f32).floor() as u16,
        (screen_pos.y / app.config.board_square_size as f32).floor() as u16,
    );
    new_pos.1 * app.config.board_squares + new_pos.0
}

pub fn board_index_to_board_xy(config: &GameConfig, index: u16) -> (u16, u16) {
    let squares = config.board_squares;
    (index % squares, index / squares)
}

pub fn board_xy_to_board_index(config: &GameConfig, xy: (u16, u16)) -> u16 {
    config.board_squares * xy.1 + xy.0
}
