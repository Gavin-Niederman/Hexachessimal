pub mod pieces;

pub(crate) trait Piece: std::fmt::Display {
    fn get_moves(&self) -> Vec<(i32, i32)>;
    fn get_pos(&self) -> (i32, i32);
    fn get_color(&self) -> Color;
    fn can_en_pessant(&self) -> bool {
        false
    }
    fn new(color: Color) -> Self
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
