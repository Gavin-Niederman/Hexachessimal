pub(crate) trait Piece: std::fmt::Display {
    fn get_moves(&self) -> (i32, i32);
    fn get_pos(&self) -> (i32, i32);
    fn get_color(&self) -> Color;
    fn can_en_pessant(&self) -> bool {
        false
    }
    fn new() -> Self
    where
        Self: Sized;
}

pub enum Color {
    White,
    Black,
}
