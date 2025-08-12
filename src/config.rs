use eframe::egui::Vec2;

#[derive(Default, Clone, Copy)]
pub struct GameConfig {
    pub board_square_size: u16,
    pub board_squares: u16,
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            board_square_size: 50,
            board_squares: 8,
        }
    }

    pub fn board_size_vec2(&self) -> Vec2 {
        Vec2::splat((self.board_square_size * self.board_squares) as f32)
    }
}
