use std::collections::HashMap;

use crate::{tile::Tile, piece::Color, piece::{pieces::*, Piece}};

pub struct Board {
    tiles: HashMap<(i32, i32), Tile>,
    width: i32,
    height: i32,
}

impl Board {
    pub fn is_piece_at_pos(&self, x: i32, y: i32) -> bool {
        if self.is_in_bounds(x, y) {
            return self.tiles.get(&(x, y)).unwrap().has_piece();
        }
        false
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        if x < self.width && x > -1 && y < self.height && y > -1 {
            return true;
        }
        false
    }

    pub fn new(width: i32, height: i32, state: &str) -> Self {
        let state = decode_fen(state);
        let mut tiles = HashMap::new();
        for taken_tile in state {
            tiles.insert((taken_tile.0, taken_tile.1), taken_tile.2);
        }
        Self {
            tiles,
            width,
            height,
        }
    }
}

pub fn decode_fen(fen: &str) -> Vec<(i32, i32, Tile)> {
    let mut tiles = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for c in fen.chars() {
        match c {
            'p' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Pawn::new(Color::Black))))));
                x += 1;
            }
            'P' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Pawn::new(Color::White))))));
                x += 1;
            }
            'r' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Rook::new(Color::Black))))));
                x += 1;
            }
            'R' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Rook::new(Color::White))))));
                x += 1;
            }
            'n' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Knight::new(Color::Black))))));
                x += 1;
            }
            'N' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Knight::new(Color::White))))));
                x += 1;
            }
            'b' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Bishop::new(Color::Black))))));
                x += 1;
            }
            'B' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Bishop::new(Color::White))))));
                x += 1;
            }
            'q' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Queen::new(Color::Black))))));
                x += 1;
            }
            'Q' => {
                tiles.push((x, y, Tile::new(Some(Box::new(Queen::new(Color::White))))));
                x += 1;
            }
            'k' => {
                tiles.push((x, y, Tile::new(Some(Box::new(King::new(Color::Black))))));
                x += 1;
            }
            'K' => {
                tiles.push((x, y, Tile::new(Some(Box::new(King::new(Color::White))))));
                x += 1;
            }
            '/' => {
                y += 1;
                x = 0;
            }
            '1'..='9' => {
                x += c.to_digit(10).unwrap() as i32;
            }
        }
    }
    tiles
}
