
pub enum Colors {
    WHITE,
    BLACK
}

pub enum PieceType {
    KING(Colors),
    QUEEN(Colors),
    BISHOP(Colors),
    KNIGHT(Colors),
    ROOK(Colors),
    PAWN(Colors),
    NULL
}

pub struct GameBoard{
    pub state: [[PieceType; 8]; 8]
}

use PieceType::{KING, QUEEN, BISHOP, KNIGHT, ROOK, PAWN, NULL};
use Colors::{WHITE, BLACK};

impl GameBoard {
    pub fn new_standard() -> Self {
        GameBoard {
            state: [
                [ROOK(BLACK), KNIGHT(BLACK), BISHOP(BLACK), QUEEN(BLACK), KING(BLACK), BISHOP(BLACK), KNIGHT(BLACK), ROOK(BLACK)],
                [PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK), PAWN(BLACK)],
                [NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL],
                [NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL],
                [NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL],
                [NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL],
                [PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE), PAWN(WHITE)],
                [ROOK(WHITE), KNIGHT(WHITE), BISHOP(WHITE), QUEEN(WHITE), KING(WHITE), BISHOP(WHITE), KNIGHT(WHITE), ROOK(WHITE)],
            ]
        }
    }
}
