use crate::{
    board::{Board, BoardExt, BoardIndex, BoardIndexExt, BoardIndexXYExt, Color, Piece, PieceType},
    move_validation::{movement::Movement, validator::validate_move},
};

const WHITE_KING_CASTLING_INDEX: BoardIndex = 60;
const BLACK_KING_CASTLING_INDEX: BoardIndex = 4;

#[derive(Default, Clone)]
pub struct GameState {
    pub board: Board,
    pub selected_square: Option<BoardIndex>,
    pub turn: Color,
    pub additional_board_data: AdditionalBoardData,
    pub awaiting_promotion: Option<(Color, BoardIndex)>,
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
            awaiting_promotion: None,
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
                let castle = self.check_for_castle(movement.clone());
                self.check_for_castle_disabling_move(movement.clone());
                self.check_for_promotion(movement.clone());
                if en_passant {
                    let modifier: i32 = if movement.movement_info.piece_color == Color::White {
                        1
                    } else {
                        -1
                    };
                    let mut destination_xy = movement.destination.to_xy();
                    destination_xy.1 = (destination_xy.1 as i32 + modifier) as u16;
                    self.board[destination_xy.to_index() as usize] = None;
                } else if castle != 0 {
                    if castle == 1 {
                        if movement.movement_info.piece_color == Color::White {
                            self.board[WHITE_KING_CASTLING_INDEX as usize] = None;
                            self.board[WHITE_KING_CASTLING_INDEX as usize + 2] =
                                Some(Piece::new(PieceType::King, Color::White));
                            self.board[WHITE_KING_CASTLING_INDEX as usize + 3] = None;
                            self.board[WHITE_KING_CASTLING_INDEX as usize + 1] =
                                Some(Piece::new(PieceType::Rook, Color::White));
                        } else {
                            self.board[BLACK_KING_CASTLING_INDEX as usize] = None;
                            self.board[BLACK_KING_CASTLING_INDEX as usize + 2] =
                                Some(Piece::new(PieceType::King, Color::Black));
                            self.board[BLACK_KING_CASTLING_INDEX as usize + 3] = None;
                            self.board[BLACK_KING_CASTLING_INDEX as usize + 1] =
                                Some(Piece::new(PieceType::Rook, Color::Black));
                        }
                    } else {
                        if movement.movement_info.piece_color == Color::White {
                            self.board[WHITE_KING_CASTLING_INDEX as usize] = None;
                            self.board[WHITE_KING_CASTLING_INDEX as usize - 2] =
                                Some(Piece::new(PieceType::King, Color::White));
                            self.board[WHITE_KING_CASTLING_INDEX as usize - 4] = None;
                            self.board[WHITE_KING_CASTLING_INDEX as usize - 1] =
                                Some(Piece::new(PieceType::Rook, Color::White));
                        } else {
                            self.board[BLACK_KING_CASTLING_INDEX as usize] = None;
                            self.board[BLACK_KING_CASTLING_INDEX as usize - 2] =
                                Some(Piece::new(PieceType::King, Color::Black));
                            self.board[BLACK_KING_CASTLING_INDEX as usize - 4] = None;
                            self.board[BLACK_KING_CASTLING_INDEX as usize - 1] =
                                Some(Piece::new(PieceType::Rook, Color::Black));
                        }
                    }
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

    /// Returns 0 if no castle, 1 if short, 2 if long
    pub fn check_for_castle(&self, m: Movement) -> u8 {
        if m.movement_info.piece_type != PieceType::King {
            return 0;
        }
        let x_distance = (m.destination.to_xy().0 as i32 - m.start.to_xy().0 as i32).abs();
        if x_distance != 2 {
            return 0;
        }
        if m.start < m.destination {
            return 1;
        }
        if m.start > m.destination {
            return 2;
        }
        return 0;
    }

    pub fn check_for_castle_disabling_move(&mut self, m: Movement) {
        if m.movement_info.piece_type == PieceType::King {
            match m.movement_info.piece_color {
                Color::White => self.additional_board_data.castling_status.0 = (false, false),
                Color::Black => self.additional_board_data.castling_status.1 = (false, false),
            }
        } else if m.destination == WHITE_KING_CASTLING_INDEX + 3
            || m.start == WHITE_KING_CASTLING_INDEX + 3
        {
            self.additional_board_data.castling_status.0.0 = false;
        } else if m.destination == WHITE_KING_CASTLING_INDEX - 4
            || m.start == WHITE_KING_CASTLING_INDEX - 4
        {
            self.additional_board_data.castling_status.0.1 = false;
        } else if m.destination == BLACK_KING_CASTLING_INDEX + 3
            || m.start == BLACK_KING_CASTLING_INDEX + 3
        {
            self.additional_board_data.castling_status.1.0 = false;
        } else if m.destination == BLACK_KING_CASTLING_INDEX - 4
            || m.start == BLACK_KING_CASTLING_INDEX - 4
        {
            self.additional_board_data.castling_status.1.1 = false;
        }
    }

    pub fn check_for_promotion(&mut self, m: Movement) {
        if m.movement_info.piece_type != PieceType::Pawn {
            self.awaiting_promotion = None;
            return;
        }
        if m.movement_info.piece_color == Color::White && m.destination.to_xy().0 == 0 {
            self.awaiting_promotion = Some((Color::White, m.destination));
        } else if m.movement_info.piece_color == Color::Black && m.destination.to_xy().0 == 7 {
            self.awaiting_promotion = Some((Color::Black, m.destination));
        } else {
            self.awaiting_promotion = None;
        }
    }

    pub fn from(board: Board, additional_board_data: AdditionalBoardData, turn: Color) -> Self {
        Self {
            board: board,
            additional_board_data: additional_board_data,
            turn: turn,
            selected_square: None,
            awaiting_promotion: None,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct AdditionalBoardData {
    // (white:short,white:long),(black:short,black:long)
    pub castling_status: ((bool, bool), (bool, bool)),
    pub en_passant_square: Option<BoardIndex>,
}
