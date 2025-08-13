use eframe::egui::{self, Pos2, Rect, Sense, Vec2, ViewportBuilder};

use crate::{
    board::{BoardExt, BoardIndex, BoardIndexExt},
    config::GameConfig,
    state::GameState,
    util::board_size_vec2,
};

mod board;
mod config;
mod rendering;
mod state;
mod util;

pub const BOARD_SQUARES: u16 = 8;
pub const BOARD_SQUARE_SIZE: u16 = 50;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2::splat(1.0))
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "guh",
        native_options,
        Box::new(|cc| Ok(Box::new(ChessApp::new(cc)))),
    )
}

#[derive(Default)]
struct ChessApp {
    config: GameConfig,
    state: GameState,
}

impl ChessApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let config = GameConfig::new();
        let state = state::GameState::new_with_default_position();
        Self {
            config: config,
            state: state,
        }
    }

    pub fn on_click(&mut self, pos: Pos2) {
        let index = BoardIndex::from_screen_click(pos);
        // either select square or move piece
        if self.state.selected_square.is_none() {
            self.state.selected_square = Some(index);
        } else {
            let piece_moved = self
                .state
                .move_piece(self.state.selected_square.unwrap(), index);
            self.state.selected_square = None;
            if piece_moved {
                self.state.switch_turn();
            }
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            rendering::resize(self, ctx);
            let board_rect = Rect::from_min_size(Pos2::ZERO, board_size_vec2());
            let response = ui.allocate_rect(board_rect, Sense::click());
            let mut painter = ui.painter_at(board_rect);
            rendering::render(self, ui, &mut painter);
            if response.clicked() {
                self.on_click(
                    (response.interact_pointer_pos().unwrap() - response.rect.min).to_pos2(),
                );
            }
        });
    }
}
