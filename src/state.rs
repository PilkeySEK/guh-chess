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
