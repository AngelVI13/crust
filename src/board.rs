use crate::defines::*;

#[derive(Debug)]
pub struct Board {
    pieces: [PieceType; BOARD_SQUARE_NUM],
}

impl Board {
    pub fn new() -> Board {
        // Returns a new board initialized to "0"/default values
        Board {
            pos: [PieceType.Empty; BOARD_SIZE],
        }
    }
}