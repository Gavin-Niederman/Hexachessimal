use std::collections::HashMap;

use crate::tile::Tile;

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

    pub fn new(width: i32, height: i32, tiles: HashMap<(i32, i32), Tile>) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }
}
