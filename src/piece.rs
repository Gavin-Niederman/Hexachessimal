use crate::{
    gamestate::GameState,
    math::{self, Vec2},
};

pub trait Piece<T>
where
    T: Piece<T>,
{
    fn get_valid_moves(&self, gamestate: &GameState<T>) -> Option<Vec<i32>>;

    fn move_peice(&mut self, offset: i32, gamestate: &GameState<T>) -> Result<(), &'static str>;

    fn is_valid_move(&self, offset: &i32, gamestate: &GameState<T>) -> bool;

    fn destroy_peice(&self, gamestate: &mut GameState<T>);

    fn get_pos(&self) -> i32;

    fn get_color(&self) -> &Color;

    fn new(color: Color, pos: i32) -> Self;
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

pub mod pieces {
    use crate::{gamestate::GameState, piece::*};

    struct Pawn {
        pos: i32,
        moveoffsets: Vec<i32>,
        color: Color,
    }

    impl<T> Piece<T> for Pawn
    where
        T: Piece<T>,
    {
        fn get_valid_moves(&self, gamestate: &GameState<T>) -> Option<Vec<i32>> {
            let mut validmoves = Vec::new();
            for i in &self.moveoffsets {
                if self.is_valid_move(i, gamestate) {
                    validmoves.push(*i);
                }
            }
            if validmoves.len() > 0 {
                Some(validmoves)
            } else {
                None
            }
        }

        fn move_peice(
            &mut self,
            offset: i32,
            gamestate: &GameState<T>,
        ) -> Result<(), &'static str> {
            if self.is_valid_move(&offset, gamestate) {
                self.pos = self.pos + offset;
                Ok(())
            } else {
                Err("Invalid move!")
            }
        }

        fn is_valid_move(&self, offset: &i32, gamestate: &GameState<T>) -> bool {
            if self.pos + *offset > 0 || self.pos + *offset < gamestate.get_board_size().0 * gamestate.get_board_size().1 {  
                if *offset == self.moveoffsets[0] {
                    if gamestate.is_tile_taken(self.pos + offset) {
                        return false;
                    } else {
                        return true;
                    }
                } else if *offset == self.moveoffsets[1] || *offset == self.moveoffsets[2] {
                    if gamestate.is_tile_taken(self.pos + offset) {
                        if *gamestate
                            .get_piece_at_pos(self.pos + offset)
                            .unwrap()
                            .get_color()
                            == self.color
                        {
                            return false;
                        } else {
                            return true;
                        }
                    }
                }
                false
            } else {
                false
            }
        }
        
        fn destroy_peice(&self, gamestate: &mut GameState<T>) {
            gamestate.remove_piece_from_pos(self.pos);
        }

        fn get_pos(&self) -> i32 {
            self.pos
        }

        fn get_color(&self) -> &Color {
            &self.color
        }

        fn new(color: Color, pos: i32) -> Self {
            //moveoffsets[0] = up
            //moveoffsets[1] = up left
            //moveoffsets[2] = up right
            let moveoffsets = vec![
                math::calculate_move_offset(Vec2 { x: 0, y: 1 }),
                math::calculate_move_offset(Vec2 { x: -1, y: 1 }),
                math::calculate_move_offset(Vec2 { x: 1, y: 1 }),
            ];
            Pawn {
                pos,
                moveoffsets,
                color,
            }
        }
    }
    struct Rook {
        pos: i32,
        moveoffsets: Vec<i32>,
        color: Color,
    }

    impl<T> Piece<T> for Rook
    where
        T: Piece<T>,
    {
        fn get_valid_moves(&self, gamestate: &GameState<T>) -> Option<Vec<i32>> {
            let mut validmoves = Vec::new();
            for i in &self.moveoffsets {
                let mut clone = i.clone();
                let vector = math::calculate_offset_from_move(&clone);
                loop {
                    if self.is_valid_move(&clone, gamestate) {
                        validmoves.push(clone);
                        if vector.x < 0 {
                            clone = clone - 1;
                        } else if vector.x > 0 {
                            clone = clone + 1;
                        } else if vector.y < 0 {
                            clone = clone - 1;
                        } else if vector.y > 0 {
                            clone = clone + 1;
                        }
                    } else {
                        break;
                    }
                }
            }
            if validmoves.len() > 0 {
                Some(validmoves)
            } else {
                None
            }
        }

        fn move_peice(
            &mut self,
            offset: i32,
            gamestate: &GameState<T>,
        ) -> Result<(), &'static str> {
            if self.is_valid_move(&offset, gamestate) {
                self.pos = self.pos + offset;
                Ok(())
            } else {
                Err("Invalid move!")
            }
        }

        fn is_valid_move(&self, offset: &i32, gamestate: &GameState<T>) -> bool {
            if self.pos + *offset > 0 || self.pos + *offset < gamestate.get_board_size().0 * gamestate.get_board_size().1 {
                if gamestate.is_tile_taken(self.pos + offset) {
                    if *gamestate
                        .get_piece_at_pos(self.pos + offset)
                        .unwrap()
                        .get_color()
                        == self.color
                    {
                        return false;
                    } else {
                        return true;
                    }
                }
                true
            } else {
                false
            }
        }

        fn destroy_peice(&self, gamestate: &mut GameState<T>) {
            gamestate.remove_piece_from_pos(self.pos);
        }

        fn get_pos(&self) -> i32 {
            self.pos
        }

        fn get_color(&self) -> &Color {
            &self.color
        }

        fn new(color: Color, pos: i32) -> Self {
            //moveoffsets[0] = up
            //moveoffsets[1] = left
            //moveoffsets[2] = right
            //moveoffsets[3] = down
            let moveoffsets = vec![
                math::calculate_move_offset(Vec2 { x: 0, y: 1 }),
                math::calculate_move_offset(Vec2 { x: -1, y: 0 }),
                math::calculate_move_offset(Vec2 { x: 1, y: 0 }),
                math::calculate_move_offset(Vec2 { x: 0, y: -1 }),
            ];
            Rook {
                pos,
                moveoffsets,
                color,
            }
        }
    }
}
