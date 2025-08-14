use crate::{
    board::{Board, BoardExt, BoardIndex, Color, PieceType},
    state::{AdditionalBoardData, GameState},
};

#[derive(Clone)]
pub struct Movement {
    pub start: BoardIndex,
    pub destination: BoardIndex,
    pub movement_info: MovementInformation,
}

impl Movement {
    pub fn from_with_state(
        start: BoardIndex,
        destination: BoardIndex,
        state: &GameState,
        board_data: AdditionalBoardData,
    ) -> Self {
        Self {
            start: start,
            destination: destination,
            movement_info: MovementInformation {
                capturing: state.board.piece_at(destination).is_some()
                    || state
                        .additional_board_data
                        .en_passant_square
                        .is_some_and(|sq| sq == destination),
                board: state.board.clone(),
                piece_type: state.board.piece_at(start).unwrap().piece_type,
                piece_color: state.board.piece_at(start).unwrap().color,
                turn: state.turn,
                board_data: board_data,
            },
        }
    }
}

#[derive(Clone)]
pub struct MovementInformation {
    pub capturing: bool,
    pub board: Board,
    pub piece_type: PieceType,
    pub piece_color: Color,
    pub turn: Color,
    pub board_data: AdditionalBoardData,
}

impl MovementInformation {
    pub fn new(
        capturing: bool,
        board: Board,
        piece_type: PieceType,
        piece_color: Color,
        turn: Color,
        board_data: AdditionalBoardData,
    ) -> Self {
        Self {
            capturing: capturing,
            board: board,
            piece_type: piece_type,
            piece_color: piece_color,
            turn: turn,
            board_data: board_data,
        }
    }
}
