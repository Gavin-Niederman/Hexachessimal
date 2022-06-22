pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

pub fn calculate_move_offset(offset: Vec2) -> i32 {
    return offset.x - offset.y * 8;
}

pub fn calculate_offset_from_move(move_offset: &i32) -> Vec2 {
    return Vec2 {
        x: move_offset % 8,
        y: move_offset / 8
    };
}