use super::Board;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn gen_board() -> Option<Board> {
    let mut board = vec![];
    let row1 = (1..=9).collect::<Vec<u8>>().shuffle(&mut thread_rng());
    let row2 =;
    board.push(row1);
    None
}
