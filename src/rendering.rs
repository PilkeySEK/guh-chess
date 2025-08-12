use eframe::egui::{
    self, Color32, CornerRadius, ImageSource, Pos2, Rect, Ui, Vec2, ViewportCommand,
};

use crate::{
    ChessApp,
    state::{Color, Piece, PieceType},
};

pub fn resize(app: &ChessApp, ctx: &egui::Context) {
    ctx.send_viewport_cmd(ViewportCommand::InnerSize(app.config.board_size_vec2()));
}

pub fn render(app: &ChessApp, ui: &mut Ui, painter: &mut egui::Painter) {
    render_board_squares(app, painter);
    render_pieces(app, ui);
}

fn render_board_squares(app: &ChessApp, painter: &mut egui::Painter) {
    painter.rect_filled(
        Rect {
            min: Pos2::ZERO,
            max: app.config.board_size_vec2().to_pos2(),
        },
        0,
        Color32::DARK_GRAY,
    );
    let mut use_white: bool = true;
    for i in 0..app.config.board_squares {
        for j in 0..app.config.board_squares {
            if use_white {
                painter.rect_filled(
                    make_rect_for_index(app, i * app.config.board_squares + j),
                    CornerRadius::ZERO,
                    Color32::WHITE,
                );
            }
            use_white = !use_white;
        }
        use_white = !use_white;
    }
    if let Some(selected_square) = app.state.selected_square {
        painter.rect_filled(
            make_rect_for_index(app, selected_square),
            CornerRadius::ZERO,
            Color32::from_rgba_unmultiplied(255, 0, 0, 128),
        );
    };
}

fn render_pieces(app: &ChessApp, ui: &mut Ui) {
    for (y, row) in app
        .state
        .board
        .chunks(app.config.board_squares as usize)
        .enumerate()
    {
        for (x, piece) in row.iter().enumerate() {
            if let Some(piece) = piece {
                render_piece_at(piece, (x as u16, y as u16), app, ui);
            }
        }
    }
}

fn render_piece_at(piece: &Piece, position: (u16, u16), app: &ChessApp, ui: &mut Ui) {
    egui::Image::new(get_piece_image(piece))
        .max_width(app.config.board_square_size as f32)
        .alt_text(format!(
            "Failed to render image of piece {piece:?} at {} {}",
            position.0, position.1
        ))
        .paint_at(
            ui,
            Rect {
                min: Pos2 {
                    x: (app.config.board_square_size * position.0) as f32,
                    y: (app.config.board_square_size * position.1) as f32,
                },
                max: Pos2 {
                    x: (app.config.board_square_size * (position.0 + 1)) as f32,
                    y: (app.config.board_square_size * (position.1 + 1)) as f32,
                },
            },
        );
}

fn get_piece_image(piece: &Piece) -> ImageSource<'static> {
    match piece {
        (PieceType::Pawn, Color::Black) => {
            egui::include_image!("./assets/chess_pieces/black_pawn.png")
        }
        (PieceType::Pawn, Color::White) => {
            egui::include_image!("./assets/chess_pieces/white_pawn.png")
        }
        (PieceType::Bishop, Color::Black) => {
            egui::include_image!("./assets/chess_pieces/black_bishop.png")
        }
        (PieceType::Bishop, Color::White) => {
            egui::include_image!("./assets/chess_pieces/white_bishop.png")
        }
        (PieceType::Knight, Color::Black) => {
            egui::include_image!("./assets/chess_pieces/black_knight.png")
        }
        (PieceType::Knight, Color::White) => {
            egui::include_image!("./assets/chess_pieces/white_knight.png")
        }
        (PieceType::Rook, Color::Black) => {
            egui::include_image!("./assets/chess_pieces/black_rook.png")
        }
        (PieceType::Rook, Color::White) => {
            egui::include_image!("./assets/chess_pieces/white_rook.png")
        }
        (PieceType::Queen, Color::Black) => {
            egui::include_image!("./assets/chess_pieces/black_queen.png")
        }
        (PieceType::Queen, Color::White) => {
            egui::include_image!("./assets/chess_pieces/white_queen.png")
        }
        (PieceType::King, Color::Black) => {
            egui::include_image!("./assets/chess_pieces/black_king.png")
        }
        (PieceType::King, Color::White) => {
            egui::include_image!("./assets/chess_pieces/white_king.png")
        }
    }
}

fn make_rect_for_index(app: &ChessApp, index: u16) -> Rect {
    let pos: (f32, f32) = (
        ((index % app.config.board_squares) * app.config.board_square_size) as f32,
        ((index / app.config.board_squares) * app.config.board_square_size) as f32,
    );
    Rect::from_min_size(
        Pos2::from(pos),
        Vec2::splat(app.config.board_square_size as f32),
    )
}
