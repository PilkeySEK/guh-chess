use eframe::egui::{self, Pos2, Rect, Sense, ViewportBuilder};

use crate::{
    board::{BoardIndex, BoardIndexExt, Piece},
    state::GameState,
    util::{
        board_size_vec2, promotion_pieces, promotion_selection_rect, promotion_selection_rect_pos,
        viewport_size_vec2,
    },
};

mod board;
mod move_validation;
mod positions;
mod rendering;
mod state;
mod util;

pub const BOARD_SQUARES: u16 = 8;
pub const BOARD_SQUARE_SIZE: u16 = 50;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_resizable(false),
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
    state: GameState,
}

impl ChessApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let state = state::GameState::new_with_default_position();
        Self { state: state }
    }

    pub fn on_click(&mut self, pos: Pos2) {
        if self.state.awaiting_promotion.is_some() {
            return;
        }
        let index = BoardIndex::from_screen_click(pos);
        // either select square or move piece
        if self.state.selected_square.is_none() {
            self.state.selected_square = Some(index);
        } else {
            self.state
                .move_piece(self.state.selected_square.unwrap(), index);
            self.state.selected_square = None;
        }
    }

    pub fn on_promotion_click(&mut self, pos: Pos2) {
        if self.state.awaiting_promotion.is_none() {
            return;
        }
        let piece = ((pos.y - promotion_selection_rect_pos().y) / (BOARD_SQUARE_SIZE as f32))
            .round() as usize;
        let piece = promotion_pieces()[piece];
        self.state.board[self.state.awaiting_promotion.unwrap().1 as usize] =
            Some(Piece::new(piece, self.state.turn));
        self.state.awaiting_promotion = None;
        self.state.switch_turn(false);
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            rendering::resize(ctx);
            let board_rect = Rect::from_min_size(Pos2::ZERO, board_size_vec2());
            let promotion_rect = promotion_selection_rect();
            let viewport_rect = Rect::from_min_size(Pos2::ZERO, viewport_size_vec2());
            let response = ui.allocate_rect(board_rect, Sense::click());
            let promotion_response = ui.allocate_rect(promotion_rect, Sense::click());
            let mut painter = ui.painter_at(viewport_rect);
            rendering::render(self, ui, &mut painter);
            if response.clicked() {
                self.on_click(
                    (response.interact_pointer_pos().unwrap() - response.rect.min).to_pos2(),
                );
            }
            if promotion_response.clicked() {
                self.on_promotion_click(
                    (promotion_response.interact_pointer_pos().unwrap()
                        - promotion_response.rect.min)
                        .to_pos2(),
                );
            }
        });
    }
}
