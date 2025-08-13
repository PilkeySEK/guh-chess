use crate::{
    board::{Board, BoardExt, BoardIndex, BoardIndexExt, BoardIndexXYExt, Color, PieceType},
    move_validation::{movement::Movement, validator::validate_move},
};

#[derive(Default)]
pub struct GameState {
    pub board: Board,
    pub turn: Color,
    pub castling_status: ((bool, bool), (bool, bool)),
    pub en_passant_square: Option<BoardIndex>,
    pub selected_square: Option<BoardIndex>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
            castling_status: ((true, true), (true, true)),
            en_passant_square: None,
            selected_square: None,
        }
    }

    pub fn new_with_default_position() -> Self {
        Self {
            board: Board::default_position(),
            turn: Color::White,
            castling_status: ((true, true), (true, true)),
            en_passant_square: None,
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
        if self.board.piece_at(start).is_none() {
            false
        } else {
            let movement = Movement::from_with_state(start, destination, self);
            if validate_move(movement.clone()) {
                self.board[destination as usize] = self.board[start as usize];
                self.board[start as usize] = None;
                self.set_en_passant_square(movement.clone());
                let en_passant = self.check_for_en_passant(movement.clone());
                if en_passant {
                    let modifier: i32 = if movement.movement_info.piece_color == Color::White {
                        1
                    } else {
                        -1
                    };
                    let mut destination_xy = movement.destination.to_xy();
                    destination_xy.1 = (destination_xy.1 as i32 + modifier) as u16;
                    self.board[destination_xy.to_index() as usize] = None;
                }
                true
            } else {
                false
            }
        }
    }

    /// Only sets the square if the moved piece was a pawn and it was moved 2 squares, else sets it to None
    pub fn set_en_passant_square(&mut self, m: Movement) {
        if m.movement_info.piece_type != PieceType::Pawn {
            self.en_passant_square = None;
            return;
        }
        let start_xy = m.start.to_xy();
        let destination_xy = m.destination.to_xy();
        let y_distance = (start_xy.1 as i32 - destination_xy.1 as i32).abs();
        if y_distance == 2 {
            let modifier: i32 = if m.movement_info.piece_color == Color::White {
                -1
            } else {
                1
            };
            let en_passant_square = (start_xy.0, (start_xy.1 as i32 + modifier) as u16);
            self.en_passant_square = Some(en_passant_square.to_index());
        } else {
            self.en_passant_square = None;
            return;
        }
    }

    /// Checks if the movement is en passant
    pub fn check_for_en_passant(&self, m: Movement) -> bool {
        // if destination square is None but capturing is true
        m.movement_info.board.piece_at(m.destination).is_none() && m.movement_info.capturing
    }
}
