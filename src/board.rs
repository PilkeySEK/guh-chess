use std::vec;

use eframe::egui::Pos2;

use crate::{BOARD_SQUARE_SIZE, BOARD_SQUARES};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Color {
    #[default]
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            color: color,
            piece_type: piece_type,
        }
    }
}

pub type Board = Vec<Option<Piece>>;

pub trait BoardExt {
    fn empty_board() -> Self;
    fn default_position() -> Board;
}

impl BoardExt for Board {
    fn empty_board() -> Self {
        vec![None; (BOARD_SQUARES * BOARD_SQUARES) as usize]
    }
    fn default_position() -> Board {
        let mut board = vec![None; (BOARD_SQUARES * BOARD_SQUARES) as usize];
        let row = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        board[0..8].copy_from_slice(&row.map(|p| Some(Piece::new(p, Color::Black))));
        board[8..16].fill(Some(Piece::new(PieceType::Pawn, Color::Black)));
        board[16..48].fill(None);
        board[48..56].fill(Some(Piece::new(PieceType::Pawn, Color::White)));
        board[56..64].copy_from_slice(&row.map(|p| Some(Piece::new(p, Color::White))));
        board
    }
}

pub type BoardIndex = u16;
pub type BoardIndexXY = (u16, u16);

pub trait BoardIndexExt {
    fn from_screen_click(pos: Pos2) -> Self;
    fn to_xy(&self) -> BoardIndexXY;
}

impl BoardIndexExt for BoardIndex {
    fn from_screen_click(pos: Pos2) -> Self {
        let xy: (u16, u16) = (
            (pos.x / BOARD_SQUARE_SIZE as f32).floor() as u16,
            (pos.y / BOARD_SQUARE_SIZE as f32).floor() as u16,
        );
        xy.1 * BOARD_SQUARES + xy.0
    }

    fn to_xy(&self) -> BoardIndexXY {
        (self % BOARD_SQUARES, self / BOARD_SQUARES)
    }
}

pub trait BoardIndexXYExt {
    fn to_index(&self) -> BoardIndex;
}

impl BoardIndexXYExt for BoardIndexXY {
    fn to_index(&self) -> BoardIndex {
        BOARD_SQUARES * self.1 + self.0
    }
}
