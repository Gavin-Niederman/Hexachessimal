mod piece;
mod math;
mod gamestate;

pub fn run() {

}

#[cfg(test)]
mod test {
    use crate::math;

    #[test]
    fn gen_offsets() {
        let offset = math::calculate_move_offset(math::Vec2{x: -1, y: 2});
        assert_eq!(offset, -17)
    }
}