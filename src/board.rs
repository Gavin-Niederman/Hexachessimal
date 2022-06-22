use crate::piece::pieces::*;

pub struct Board {
    pub pawns: Vec<Pawn>,
    pub rooks: Vec<Rook>,
    pub bishops: Vec<Bishop>,
    pub knights: Vec<Knight>,
    pub queens: Vec<Queen>,
    pub kings: Vec<King>,
}