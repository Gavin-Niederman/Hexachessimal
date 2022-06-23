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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Pawn(p) => {
                if *p.get_color() == Color::White {
                    write!(f, "\x1B[1;97mP\x1B[0m")
                } else {
                    write!(f, "\x1B[38;5;238mP\x1B[0m")
                }
            },
            Tile::Rook(r) => {
                if *r.get_color() == Color::White {
                    write!(f, "\x1B[1;97mR\x1B[0m")
                } else {
                    write!(f, "\x1B[38;5;238mR\x1B[0m")
                }
            },
            Tile::Bishop(b) => {
                if *b.get_color() == Color::White {
                    write!(f, "\x1B[1;97mB\x1B[0m")
                } else {
                    write!(f, "\x1B[38;5;238mB\x1B[0m")
                }
            },
            Tile::Knight(k) => {
                if *k.get_color() == Color::White {
                    write!(f, "\x1B[1;97mN\x1B[0m")
                } else {
                    write!(f, "\x1B[38;5;238mN\x1B[0m")
                }
            },
            Tile::Queen(q) => {
                if *q.get_color() == Color::White {
                    write!(f, "\x1B[1;97mQ\x1B[0m")
                } else {
                    write!(f, "\x1B[38;5;238mQ\x1B[0m")
                }
            },
            Tile::King(k) => {
                if *k.get_color() == Color::White {
                    write!(f, "\x1B[1;97mK\x1B[0m")
                } else {
                    write!(f, "\x1B[38;5;238mK\x1B[0m")
                }
            },
        }
    }
}