use crate::{piece::{Piece, Color}, board::Board};

pub struct GameState {
    boardheight: i32,
    boardwidth: i32,
    board: Board
}

impl GameState {
    pub fn get_board_size(&self) -> (i32, i32) {
        (self.boardwidth, self.boardheight)
    }

    pub fn is_tile_taken(&self, pos: i32) -> bool {
        for piece in &self.board.pawns {
            if piece.get_pos() == pos {
                return true;
            }
        }
        for piece in &self.board.rooks {
            if piece.get_pos() == pos {
                return true;
            }
        }
        for piece in &self.board.bishops {
            if piece.get_pos() == pos {
                return true;
            }
        }
        for piece in &self.board.knights {
            if piece.get_pos() == pos {
                return true;
            }
        }
        for piece in &self.board.queens {
            if piece.get_pos() == pos {
                return true;
            }
        }
        for piece in &self.board.kings {
            if piece.get_pos() == pos {
                return true;
            }
        }
        false
    }

    pub fn get_piece_color_at_pos(&self, pos: i32) -> Result<&Color, String> {
        for piece in &self.board.pawns {
            if piece.get_pos() == pos {
                return Ok(piece.get_color());
            }
        }
        for piece in &self.board.rooks {
            if piece.get_pos() == pos {
                return Ok(piece.get_color());
            }
        }
        for piece in &self.board.bishops {
            if piece.get_pos() == pos {
                return Ok(piece.get_color());
            }
        }
        for piece in &self.board.knights {
            if piece.get_pos() == pos {
                return Ok(piece.get_color());
            }
        }
        for piece in &self.board.queens {
            if piece.get_pos() == pos {
                return Ok(piece.get_color());
            }
        }
        for piece in &self.board.kings {
            if piece.get_pos() == pos {
                return Ok(piece.get_color());
            }
        }
        Err(String::from("No piece at position:") + &pos.to_string())
    }

    pub fn remove_piece_from_pos(&mut self, pos: i32) {
        self.board.pawns.retain(|p| p.get_pos() != pos);
        self.board.rooks.retain(|p| p.get_pos() != pos);
        self.board.bishops.retain(|p| p.get_pos() != pos);
        self.board.knights.retain(|p| p.get_pos() != pos);
        self.board.queens.retain(|p| p.get_pos() != pos);
        self.board.kings.retain(|p| p.get_pos() != pos);
    }
}