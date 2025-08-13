use crate::{
    board::{BoardExt, BoardIndexExt, Color, PieceType},
    move_validation::movement::Movement,
};

pub fn validate_move(m: Movement) -> bool {
    if m.movement_info.piece_color != m.movement_info.turn {
        false
    } else {
        let destination_piece = m.movement_info.board.piece_at(m.destination);
        let start_xy = m.start.to_xy();
        let destination_xy = m.destination.to_xy();
        let x_distance = (start_xy.0 as i32 - destination_xy.0 as i32).abs();
        let y_distance = (start_xy.1 as i32 - destination_xy.1 as i32).abs();
        match m.movement_info.piece_type {
            PieceType::Pawn => {
                let modifier: i32 = if m.movement_info.piece_color == Color::White {
                    -1
                } else {
                    1
                };
                if start_xy.0 != destination_xy.0 {
                    if !m.movement_info.capturing {
                        false
                    } else if !(start_xy.0 as i32 + 1 == destination_xy.0 as i32
                        || start_xy.0 as i32 - 1 == destination_xy.0 as i32)
                    {
                        false
                    } else {
                        if start_xy.1 as i32 + modifier == destination_xy.1 as i32 {
                            true
                        } else {
                            false
                        }
                    }
                } else {
                    if start_xy.1 as i32 + modifier == destination_xy.1 as i32 {
                        m.movement_info.board.piece_at(m.destination).is_none()
                    } else if start_xy.1 as i32 + modifier * 2 == destination_xy.1 as i32 {
                        if m.movement_info.piece_color == Color::White && start_xy.1 != 6 {
                            false
                        } else if m.movement_info.piece_color == Color::Black && start_xy.1 != 1 {
                            false
                        } else {
                            m.movement_info.board.piece_at(m.destination).is_none()
                        }
                    } else {
                        false
                    }
                }
            }
            PieceType::King => {
                if x_distance > 1 || y_distance > 1 || (x_distance == 0 && y_distance == 0) {
                    false
                } else if destination_piece.is_none() {
                    true
                } else if destination_piece.unwrap().color == m.movement_info.piece_color {
                    false
                } else {
                    true
                }
            }
            PieceType::Knight => {
                if x_distance == 1 && y_distance == 2 || x_distance == 2 && y_distance == 1 {
                    if destination_piece.is_none() {
                        true
                    } else if destination_piece.unwrap().color == m.movement_info.piece_color {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            _ => todo!(),
        }
    }
}
