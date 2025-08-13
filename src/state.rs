use crate::board::{Board, BoardExt, BoardIndex, Color};

#[derive(Default)]
pub struct GameState {
    pub board: Board,
    pub turn: Color,
    pub castling_status: ((bool, bool), (bool, bool)),
    pub selected_square: Option<BoardIndex>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
            castling_status: ((true, true), (true, true)),
            selected_square: None,
        }
    }

    pub fn new_with_default_position() -> Self {
        Self {
            board: Board::default_position(),
            turn: Color::White,
            castling_status: ((true, true), (true, true)),
            selected_square: None,
        }
    }

    pub fn switch_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }

    /// Moves the piece from `start` to `destination`. This function validates the move fully.
    /// Returns `false` if the move is invalid and the move was not performed.
    /// Returns `true` if the move is valid and the piece was moved.
    pub fn move_piece(&mut self, start: BoardIndex, destination: BoardIndex) -> bool {
        self.board[destination as usize] = self.board[start as usize];
        true
    }
}
