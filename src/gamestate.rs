use crate::piece::Piece;

pub struct GameState<T>
    where T: Piece<T>
{
    boardheight: i32,
    boardwidth: i32,
    pieces: Vec<T>
}

impl<T> GameState<T>
    where T: Piece<T>
{
    pub fn get_board_size(&self) -> (i32, i32) {
        (self.boardwidth, self.boardheight)
    }

    pub fn is_tile_taken(&self, pos: i32) -> bool {
        for piece in &self.pieces {
            if piece.get_pos() == pos {
                return true;
            }
        }
        false
    }

    pub fn get_piece_at_pos(&self, pos: i32) -> Result<&T, String> {
        for piece in &self.pieces {
            if piece.get_pos() == pos {
                return Ok(piece);
            }
        }
        Err(String::from("No piece at position:") + &pos.to_string())
    }

    pub fn remove_piece_from_pos(&mut self, pos: i32) {
        self.pieces.retain(|p| p.get_pos() != pos);
    }
}