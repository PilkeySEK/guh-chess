use std::ops::Range;

use crate::{
    BOARD_SQUARES,
    board::{
        Board, BoardExt, BoardIndex, BoardIndexExt, BoardIndexXY, BoardIndexXYExt, Color, Piece,
        PieceType,
    },
    move_validation::movement::Movement,
    state::AdditionalBoardData,
};

pub fn validate_move(m: Movement) -> bool {
    if m.movement_info.piece_color != m.movement_info.turn {
        false
    } else {
        generate_piece_map(
            &m.movement_info.board,
            &m.movement_info.board_data,
            m.movement_info.turn,
            m.start,
        )
        .contains(&m.destination)
    }
}

/// Creates a list of all possible destinations
pub fn generate_piece_map(
    board: &Board,
    board_data: &AdditionalBoardData,
    turn: Color,
    piece_index: BoardIndex,
) -> Vec<BoardIndex> {
    let mut piece_map: Vec<BoardIndex> = Vec::new();

    let piece = board.piece_at(piece_index);
    if piece.is_none() {
        return piece_map;
    }

    let piece = piece.unwrap();
    if piece.color != turn {
        return piece_map;
    }
    let xy_index = piece_index.to_xy();
    match piece.piece_type {
        PieceType::Pawn => {
            let mut may_push_1_square = false;
            let color_modifier = if piece.color == Color::White { -1 } else { 1 };
            if (xy_index.1 as i32 + color_modifier) >= 0 {
                let one_ahead_index =
                    (xy_index.0, (xy_index.1 as i32 + color_modifier) as u16).to_index();
                let one_ahead = board.piece_at(one_ahead_index);
                if one_ahead.is_none() {
                    may_push_1_square = true;
                    piece_map.push(one_ahead_index);
                }

                let one_ahead_xy = one_ahead_index.to_xy();
                if one_ahead_xy.0 > 0 {
                    let take_xy = (one_ahead_xy.0 - 1, one_ahead_xy.1);
                    if board
                        .piece_at(take_xy.to_index())
                        .is_some_and(|p| p.color != piece.color)
                    {
                        piece_map.push(take_xy.to_index());
                    } else if board_data
                        .en_passant_square
                        .is_some_and(|en_passant_square| en_passant_square == take_xy.to_index())
                    {
                        piece_map.push(take_xy.to_index());
                    }
                }
                if one_ahead_xy.0 < BOARD_SQUARES - 1 {
                    let take_xy = (one_ahead_xy.0 + 1, one_ahead_xy.1);
                    if board
                        .piece_at(take_xy.to_index())
                        .is_some_and(|p| p.color != piece.color)
                    {
                        piece_map.push(take_xy.to_index());
                    } else if board_data
                        .en_passant_square
                        .is_some_and(|en_passant_square| en_passant_square == take_xy.to_index())
                    {
                        piece_map.push(take_xy.to_index());
                    }
                }
            }

            let may_push_2_squares = if piece.color == Color::White {
                xy_index.1 == 6
            } else {
                xy_index.1 == 1
            };
            if (xy_index.1 as i32 + color_modifier * 2) >= 0
                && may_push_2_squares
                && may_push_1_square
            {
                let two_ahead_index =
                    (xy_index.0, (xy_index.1 as i32 + color_modifier * 2) as u16).to_index();
                let two_ahead = board.piece_at(two_ahead_index);
                if two_ahead.is_none() {
                    piece_map.push(two_ahead_index);
                }
            }
        }
        PieceType::King => {
            let adjacent_squares = adjacent_king_squares(piece_index);
            for ele in adjacent_squares {
                if board.piece_at(ele).is_none_or(|p| p.color != piece.color) {
                    piece_map.push(ele);
                }
            }
        }
        PieceType::Knight => {
            let adjacent_squares = adjacent_knight_squares(piece_index);
            for ele in adjacent_squares {
                if board.piece_at(ele).is_none_or(|p| p.color != piece.color) {
                    piece_map.push(ele);
                }
            }
        }
        PieceType::Rook => {
            let modifiers = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for ele in get_piece_map_from_modifiers(board, piece_index, piece, &modifiers) {
                piece_map.push(ele);
            }
        }
        PieceType::Bishop => {
            let modifiers = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
            for ele in get_piece_map_from_modifiers(board, piece_index, piece, &modifiers) {
                piece_map.push(ele);
            }
        }
        PieceType::Queen => {
            let modifiers = [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
            ];
            for ele in get_piece_map_from_modifiers(board, piece_index, piece, &modifiers) {
                piece_map.push(ele);
            }
        }
        _ => {}
    }
    piece_map
}

fn adjacent_king_squares(index: BoardIndex) -> Vec<BoardIndex> {
    let modifiers = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    adjacent_squares_from_modifiers(index, &modifiers)
}

fn adjacent_knight_squares(index: BoardIndex) -> Vec<BoardIndex> {
    let modifiers = [
        (2, 1),
        (1, 2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (2, -1),
        (1, -2),
        (-1, 2),
    ];
    adjacent_squares_from_modifiers(index, &modifiers)
}

fn adjacent_squares_from_modifiers(index: BoardIndex, modifiers: &[(i32, i32)]) -> Vec<BoardIndex> {
    let xy_index = index.to_xy();
    let mut adjacent = Vec::new();
    for ele in modifiers {
        let new_index = (xy_index.0 as i32 + ele.0, xy_index.1 as i32 + ele.1);
        if new_index.0 < 0
            || new_index.0 >= BOARD_SQUARES as i32
            || new_index.1 < 0
            || new_index.1 >= BOARD_SQUARES as i32
        {
            continue;
        }
        adjacent.push((new_index.0 as u16, new_index.1 as u16).to_index());
    }
    adjacent
}

fn validate_i32_pos(pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < BOARD_SQUARES as i32 && pos.1 < BOARD_SQUARES as i32
}

fn get_piece_map_from_modifiers(
    board: &Board,
    index: BoardIndex,
    piece: Piece,
    modifiers: &[(i32, i32)],
) -> Vec<BoardIndex> {
    let mut piece_map = Vec::new();
    let xy_index = index.to_xy();
    for modifier in modifiers {
        let mut current_pos = (
            xy_index.0 as i32 + modifier.0,
            xy_index.1 as i32 + modifier.1,
        );
        let mut limit = BOARD_SQUARES as i32;
        while limit >= 0 {
            if !validate_i32_pos(current_pos) {
                break;
            }
            let current_pos_index = (current_pos.0 as u16, current_pos.1 as u16).to_index();
            let current_piece = board.piece_at(current_pos_index);
            if current_piece.is_none() {
                piece_map.push(current_pos_index);
            } else if current_piece.is_some_and(|p| p.color != piece.color) {
                piece_map.push(current_pos_index);
                break;
            } else {
                break;
            }
            current_pos.0 += modifier.0;
            current_pos.1 += modifier.1;
            limit -= 1;
        }
    }
    piece_map
}
