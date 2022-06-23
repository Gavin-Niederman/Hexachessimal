mod piece;
mod math;
mod gamestate;
mod tile;

pub fn run() {
    let mut gamestate = gamestate::GameState::new(8, 8);
    println!("{}", gamestate);
}

#[cfg(test)]
mod test {
    use crate::{math, tile::Tile, piece::{pieces::Pawn, Piece, Color}};

    #[test]
    fn gen_offsets() {
        let offset = math::calculate_move_offset(math::Vec2{x: -1, y: 2});
        assert_eq!(offset, -17)
    }

    #[test]
    fn print_tile() {
        let tile = Tile::Pawn(Pawn::new(Color::White, 0));
        println!("{}", tile);
    }

    // #[test]
    // fn move_off_board() {
    //     let gamestate = GameState {
    //         boardheight: 8,
    //         boardwidth: 8,
    //         board: Board {
    //             pawns: vec![pieces::Pawn {
    //                 pos: 0,
    //                 moveoffsets: vec![math::calculate_move_offset(math::Vec2{x: 0, y: 1}),
    //                                   math::calculate_move_offset(math::Vec2{x: -1, y: 1}),
    //                                   math::calculate_move_offset(math::Vec2{x: 1, y: 1})],
    //                 color: Color::White,
    //             }],
    //             rooks: vec![],
    //             bishops: vec![],
    //             knights: vec![],
    //             queens: vec![],
    //             kings: vec![],
    //         },
    //     };
    // }
}