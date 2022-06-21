pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

pub fn calculate_move_offset(offset: Vec2) -> i32 {
    return offset.x - offset.y * 8;
}