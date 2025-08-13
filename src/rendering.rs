use eframe::egui::{
    self, Color32, CornerRadius, ImageSource, Pos2, Rect, Ui, Vec2, ViewportCommand,
};

use crate::{
    BOARD_SQUARE_SIZE, BOARD_SQUARES, ChessApp,
    board::{Color, Piece, PieceType},
    util::board_size_vec2,
};

pub fn resize(app: &ChessApp, ctx: &egui::Context) {
    ctx.send_viewport_cmd(ViewportCommand::InnerSize(board_size_vec2()));
}

pub fn render(app: &ChessApp, ui: &mut Ui, painter: &mut egui::Painter) {
    render_board_squares(app, painter);
    render_pieces(app, ui);
}

fn render_board_squares(app: &ChessApp, painter: &mut egui::Painter) {
    painter.rect_filled(
        Rect {
            min: Pos2::ZERO,
            max: board_size_vec2().to_pos2(),
        },
        0,
        Color32::DARK_GRAY,
    );
    let mut use_white: bool = true;
    for i in 0..BOARD_SQUARES {
        for j in 0..BOARD_SQUARES {
            if use_white {
                painter.rect_filled(
                    make_rect_for_index(app, i * BOARD_SQUARES + j),
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
    for (y, row) in app.state.board.chunks(BOARD_SQUARES as usize).enumerate() {
        for (x, piece) in row.iter().enumerate() {
            if let Some(piece) = piece {
                render_piece_at(piece, (x as u16, y as u16), app, ui);
            }
        }
    }
}

fn render_piece_at(piece: &Piece, position: (u16, u16), app: &ChessApp, ui: &mut Ui) {
    egui::Image::new(get_piece_image(piece))
        .max_width(BOARD_SQUARE_SIZE as f32)
        .alt_text(format!(
            "Failed to render image of piece type={} color={} at {} {}",
            piece.piece_type as i8, piece.color as i8, position.0, position.1
        ))
        .paint_at(
            ui,
            Rect {
                min: Pos2 {
                    x: (BOARD_SQUARE_SIZE * position.0) as f32,
                    y: (BOARD_SQUARE_SIZE * position.1) as f32,
                },
                max: Pos2 {
                    x: (BOARD_SQUARE_SIZE * (position.0 + 1)) as f32,
                    y: (BOARD_SQUARE_SIZE * (position.1 + 1)) as f32,
                },
            },
        );
}

fn get_piece_image(piece: &Piece) -> ImageSource<'static> {
    match piece.piece_type {
        PieceType::Pawn => match piece.color {
            Color::White => egui::include_image!("./assets/chess_pieces/white_pawn.png"),
            Color::Black => egui::include_image!("./assets/chess_pieces/black_pawn.png"),
        },
        PieceType::Bishop => match piece.color {
            Color::White => egui::include_image!("./assets/chess_pieces/white_bishop.png"),
            Color::Black => egui::include_image!("./assets/chess_pieces/black_bishop.png"),
        },
        PieceType::Knight => match piece.color {
            Color::White => egui::include_image!("./assets/chess_pieces/white_knight.png"),
            Color::Black => egui::include_image!("./assets/chess_pieces/black_knight.png"),
        },
        PieceType::Rook => match piece.color {
            Color::White => egui::include_image!("./assets/chess_pieces/white_rook.png"),
            Color::Black => egui::include_image!("./assets/chess_pieces/black_rook.png"),
        },
        PieceType::Queen => match piece.color {
            Color::White => egui::include_image!("./assets/chess_pieces/white_queen.png"),
            Color::Black => egui::include_image!("./assets/chess_pieces/black_queen.png"),
        },
        PieceType::King => match piece.color {
            Color::White => egui::include_image!("./assets/chess_pieces/white_king.png"),
            Color::Black => egui::include_image!("./assets/chess_pieces/black_king.png"),
        },
    }
}

fn make_rect_for_index(app: &ChessApp, index: u16) -> Rect {
    let pos: (f32, f32) = (
        ((index % BOARD_SQUARES) * BOARD_SQUARE_SIZE) as f32,
        ((index / BOARD_SQUARES) * BOARD_SQUARE_SIZE) as f32,
    );
    Rect::from_min_size(Pos2::from(pos), Vec2::splat(BOARD_SQUARE_SIZE as f32))
}
