use super::{Piece, Color};

pub struct Pawn {
    color: Color,
    pos: (i32, i32),
    can_en_pessant: bool,
    has_moved: bool,
}

impl Piece for Pawn {
    fn get_moves(&self) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        if self.color == Color::White {
            moves.push((self.pos.0, self.pos.1 + 1));
            if !self.has_moved {
                moves.push((self.pos.0, self.pos.1 + 2));
            }
        } else {
            moves.push((self.pos.0, self.pos.1 - 1));
            if !self.has_moved {
                moves.push((self.pos.0, self.pos.1 - 2));
            }
        }
        moves
    }

    fn get_pos(&self) -> (i32, i32) {
        self.pos
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn can_en_pessant(&self) -> bool {
        self.can_en_pessant
    }

    fn new(color: Color) -> Self
    where
        Self: Sized,
    {
        Self {
            color,
            pos: (0, 0),
            can_en_pessant: false,
            has_moved: false,
        }
    }
}

impl std::fmt::Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P")
    }
}