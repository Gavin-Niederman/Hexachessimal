use crate::{math::{Vec2, calculate_move_offset}, gamestate::GameState};

pub trait Piece<T> 
    where T: Piece<T>
{
    fn get_valid_moves(&self, gamestate: GameState<T>) -> Option<Vec<i32>>;
    
    fn move_peice(&mut self, offset: i32, gamestate: GameState<T>) -> Result<(), &'static str>;

    fn is_valid_move(&self, offset: i32, gamestate: GameState<T>) -> bool;

    fn destroy_peice(&self);

    fn get_pos(&self) -> i32;

    fn get_color(&self) -> &Color;

    fn new(color: Color, pos: i32) -> Self;
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black
}

pub mod pieces {
    use crate::{piece::*, gamestate::GameState};

    struct Pawn {
        pos: i32,
        moveoffsets: Vec<i32>,
        color: Color
    }

    impl<T> Piece<T> for Pawn 
        where T: Piece<T>
    {
        fn get_valid_moves(&self, gamestate: GameState<T>) -> Option<Vec<i32>> {
            let mut validmoves = Vec::new();
            for i in &self.moveoffsets {
                if self.is_valid_move(*i, gamestate) {
                    validmoves.push(*i);
                }
            }
            if validmoves.len() > 0 {
                Some(validmoves)
            } else {
                None
            }
        }

        fn move_peice(&mut self, offset: i32, gamestate: GameState<T>) -> Result<(), &'static str> {
            if self.is_valid_move(offset, gamestate) {
                self.pos = self.pos + offset;
                Ok(())
            } else {
                Err("Invalid move!")
            }
        }

        fn is_valid_move(&self, offset: i32, gamestate: GameState<T>) -> bool {
            let offsetpos = self.pos + offset;

            if offset == self.moveoffsets[0] && !gamestate.is_tile_taken(offsetpos) {
                true
            } else if offset == self.moveoffsets[1] && gamestate.is_tile_taken(offsetpos) {
                if *gamestate.get_piece_at_pos(offsetpos).unwrap().get_color() != *self.get_color() {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn destroy_peice(&self) {
            todo!()
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
            let moveoffsets = vec![calculate_move_offset(Vec2 {x: 0, y: 1}),
            calculate_move_offset(Vec2 {x: -1, y: 1}),
            calculate_move_offset(Vec2 {x: 1, y: 1})
            ];
            Pawn {
                pos,
                moveoffsets,
                color
            }
        }
    }
}