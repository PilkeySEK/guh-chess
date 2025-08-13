use crate::{
    ChessApp,
    config::GameConfig,
    util::{board_index_to_board_xy, board_xy_to_board_index},
};

#[derive(Default)]
pub struct GameState {
    pub board: Vec<Option<Piece>>,
    pub turn: Color,
    pub selected_square: Option<u16>,
}

impl GameState {
    pub fn new(board_size: u16) -> Self {
        let mut state = Self {
            board: vec![None; (board_size * board_size) as usize],
            turn: Color::White,
            selected_square: Some(10),
        };
        state.board[1] = Some((PieceType::Bishop, Color::White));
        state.board[10] = Some((PieceType::King, Color::Black));
        state
    }

    pub fn set_default_position(mut self) -> Self {
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

        self.board[0..8].copy_from_slice(&row.map(|p| Some((p, Color::Black))));
        self.board[8..16].fill(Some((PieceType::Pawn, Color::Black)));
        self.board[16..48].fill(None);
        self.board[48..56].fill(Some((PieceType::Pawn, Color::White)));
        self.board[56..64].copy_from_slice(&row.map(|p| Some((p, Color::White))));
        self
    }

    // Returns true if the movement is valid and the piece has been moved, and false if not (false also means the piece has not been moved)
    pub fn move_piece(&mut self, config: &GameConfig, from: u16, to: u16) -> bool {
        let from = from as usize;
        if self.board.len() <= from {
            false
        }
        // origin square has no piece
        else if self.board[from].is_none() {
            false
        }
        // piece has the wrong color (if it's white's turn, they can't move a black piece)
        else if self.board[from].unwrap().1 != self.turn {
            false
        } else {
            let piece = self.board[from].unwrap();
            let valid = self.validate_piece_movement(config, piece, from as u16, to);
            if valid {
                self.board[from] = None;
                self.board[to as usize] = Some(piece);
                true
            } else {
                false
            }
        }
    }

    fn validate_piece_movement(
        &self,
        config: &GameConfig,
        piece: (PieceType, Color),
        from: u16,
        to: u16,
    ) -> bool {
        true
    }

    pub fn switch_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Color {
    Black,
    #[default]
    White,
}

pub type Piece = (PieceType, Color);
