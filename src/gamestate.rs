use crate::{piece::Color, tile::Tile};

pub struct GameState {
    boardheight: i32,
    boardwidth: i32,
    board: Vec<Tile>
}

impl GameState {
    pub fn get_board_size(&self) -> (i32, i32) {
        (self.boardwidth, self.boardheight)
    }

    pub fn is_tile_taken(&self, pos: i32) -> bool {
        for tile in &self.board {
            if tile.get_pos() == pos {
                return true;
            }
        }
        false
    }

    pub fn get_piece_color_at_pos(&self, pos: i32) -> Result<&Color, String> {
        for tile in &self.board {
            if tile.get_pos() == pos {
                return Ok(tile.get_color());
            }
        }
        Err(String::from("No piece at position:") + &pos.to_string())
    }

    pub fn remove_piece_from_pos(&mut self, pos: i32) {
        self.board.retain(|p| p.get_pos() != pos);
    }
}