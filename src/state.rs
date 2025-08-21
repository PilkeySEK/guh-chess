use crate::{
    board::{Board, BoardExt, BoardIndex, BoardIndexExt, BoardIndexXYExt, Color, PieceType},
    move_validation::{movement::Movement, validator::validate_move},
};

#[derive(Default, Clone)]
pub struct GameState {
    pub board: Board,
    pub selected_square: Option<BoardIndex>,
    pub turn: Color,
    pub additional_board_data: AdditionalBoardData,
}

impl GameState {
    pub fn new_with_default_position() -> Self {
        Self {
            board: Board::default_position(),
            turn: Color::White,
            additional_board_data: AdditionalBoardData {
                castling_status: ((true, true), (true, true)),
                en_passant_square: None,
            },
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
    /// Also switches the turn.
    fn _move_piece(
        &mut self,
        start: BoardIndex,
        destination: BoardIndex,
        bypass_validation: bool,
    ) -> bool {
        if self.board.piece_at(start).is_none() {
            false
        } else {
            let movement =
                Movement::from_with_state(start, destination, self, self.additional_board_data);
            let validation = if bypass_validation {
                true
            } else {
                validate_move(movement.clone())
            };
            if validation {
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
                self.switch_turn();
                true
            } else {
                false
            }
        }
    }

    pub fn move_piece(&mut self, start: BoardIndex, destination: BoardIndex) -> bool {
        self._move_piece(start, destination, false)
    }

    pub fn move_piece_bypass_validation(
        &mut self,
        start: BoardIndex,
        destination: BoardIndex,
    ) -> bool {
        self._move_piece(start, destination, true)
    }

    /// Only sets the square if the moved piece was a pawn and it was moved 2 squares, else sets it to None
    pub fn set_en_passant_square(&mut self, m: Movement) {
        if m.movement_info.piece_type != PieceType::Pawn {
            self.additional_board_data.en_passant_square = None;
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
            self.additional_board_data.en_passant_square = Some(en_passant_square.to_index());
        } else {
            self.additional_board_data.en_passant_square = None;
            return;
        }
    }

    /// Checks if the movement is en passant
    pub fn check_for_en_passant(&self, m: Movement) -> bool {
        // if destination square is None but capturing is true
        m.movement_info.board.piece_at(m.destination).is_none() && m.movement_info.capturing
    }

    pub fn from(board: Board, additional_board_data: AdditionalBoardData, turn: Color) -> Self {
        Self {
            board: board,
            additional_board_data: additional_board_data,
            turn: turn,
            selected_square: None,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct AdditionalBoardData {
    #[allow(dead_code)]
    pub castling_status: ((bool, bool), (bool, bool)),
    pub en_passant_square: Option<BoardIndex>,
}
