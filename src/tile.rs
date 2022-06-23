use crate::piece::{pieces::*, Piece, Color};

pub enum Tile {
    Pawn(Pawn),
    Rook(Rook),
    Bishop(Bishop),
    Knight(Knight),
    Queen(Queen),
    King(King),
}

impl Tile {
    pub fn get_pos(&self) -> i32 {
        match self {
            Tile::Pawn(p) => (*p).get_pos(),
            Tile::Rook(r) => (*r).get_pos(),
            Tile::Bishop(b) => (*b).get_pos(),
            Tile::Knight(k) => (*k).get_pos(),
            Tile::Queen(q) => (*q).get_pos(),
            Tile::King(k) => (*k).get_pos(),
        }
    }

    pub fn get_color(&self) -> &Color {
        match self {
            Tile::Pawn(p) => p.get_color(),
            Tile::Rook(r) => r.get_color(),
            Tile::Bishop(b) => b.get_color(),
            Tile::Knight(k) => k.get_color(),
            Tile::Queen(q) => q.get_color(),
            Tile::King(k) => k.get_color(),
        }
    }
}