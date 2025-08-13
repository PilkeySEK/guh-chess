use eframe::egui::{self, Pos2, Rect, Sense, Vec2, ViewportBuilder};

use crate::{config::GameConfig, util::screen_pos_to_board_index};
mod config;
mod rendering;
mod state;
mod util;

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
    config: config::GameConfig,
    state: state::GameState,
}

impl ChessApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let config = GameConfig::new();
        Self {
            config: config,
            state: state::GameState::new(config.board_squares).set_default_position(),
        }
    }

    pub fn on_click(&mut self, pos: Pos2) {
        let index = screen_pos_to_board_index(self, pos);
        self.state.selected_square = Some(index);
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            rendering::resize(self, ctx);
            let board_rect = Rect::from_min_size(Pos2::ZERO, self.config.board_size_vec2());
            let mut response = ui.allocate_rect(board_rect, Sense::click());
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
