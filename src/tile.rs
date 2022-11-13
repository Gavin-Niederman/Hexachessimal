use crate::piece::Piece;

pub struct Tile {
    piece: Option<Box<dyn Piece>>,
}

impl Tile {
    pub fn has_piece(&self) -> bool {
        self.piece.is_some()
    }

    pub fn new(piece: Option<Box<dyn Piece>>) -> Self {
        Self { piece: None }
    }
}
