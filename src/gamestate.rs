use crate::{piece::{Color, Piece, pieces}, tile::Tile};

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

    pub fn get_tile_at_pos(&self, pos: i32) -> Result<&Tile, String> {
        for tile in &self.board {
            if tile.get_pos() == pos {
                return Ok(tile);
            }
        }
        Err(String::from("No tile at position:") + &pos.to_string())
    }

    pub fn remove_piece_from_pos(&mut self, pos: i32) {
        self.board.retain(|p| p.get_pos() != pos);
    }

    pub fn new(boardheight: i32, boardwidth: i32) -> GameState {
        let board = vec![
        Tile::Rook(pieces::Rook::new(Color::White, 0)),
        Tile::Knight(pieces::Knight::new(Color::White, 1)),
        Tile::Bishop(pieces::Bishop::new(Color::White, 2)),
        Tile::Queen(pieces::Queen::new(Color::White, 3)),
        Tile::King(pieces::King::new(Color::White, 4)),
        Tile::Bishop(pieces::Bishop::new(Color::White, 5)),
        Tile::Knight(pieces::Knight::new(Color::White, 6)),
        Tile::Rook(pieces::Rook::new(Color::White, 7)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 8)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 9)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 10)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 11)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 12)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 13)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 14)),
        Tile::Pawn(pieces::Pawn::new(Color::White, 15)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 48)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 49)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 50)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 51)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 52)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 53)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 54)),
        Tile::Pawn(pieces::Pawn::new(Color::Black, 55)),
        Tile::Rook(pieces::Rook::new(Color::Black, 56)),
        Tile::Knight(pieces::Knight::new(Color::Black, 57)),
        Tile::Bishop(pieces::Bishop::new(Color::Black, 58)),
        Tile::Queen(pieces::Queen::new(Color::Black, 59)),
        Tile::King(pieces::King::new(Color::Black, 60)),
        Tile::Bishop(pieces::Bishop::new(Color::Black, 61)),
        Tile::Knight(pieces::Knight::new(Color::Black, 62)),
        Tile::Rook(pieces::Rook::new(Color::Black, 63)),
        ];
        GameState { boardheight, boardwidth, board }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐ ")?;
        for y in 0..self.boardheight {
            write!(f, "{} ", 8 - y)?;
            for x in 0..self.boardwidth {
                if self.is_tile_taken(y * self.boardwidth + x) {
                    write!(f, "│ {} ", self.get_tile_at_pos(y * self.boardwidth + x).unwrap())?;
                } else {
                    write!(f, "│   ")?;
                }
            }
            writeln!(f, "│")?;
            if y != self.boardheight - 1 {
                writeln!(f, "  ├───┼───┼───┼───┼───┼───┼───┼───┤ ")?;
            }
        }
        writeln!(f, "  └───┴───┴───┴───┴───┴───┴───┴───┘")?;
        writeln!(f, "    A   B   C   D   E   F   G   H")
    }
}