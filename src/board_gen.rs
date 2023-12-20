use super::Board;
use super::board::Tile;
use super::Error;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn gen_board() -> Result<Board, Error> {
    let mut row1 = make_tile((1..=9).collect::<Vec<u8>>());

    row1.shuffle(&mut thread_rng());
    let mut row2 = row1.clone();
    row2.rotate_right(3);
    let mut row3 = row1.clone();
    row3.rotate_right(6);

    let mut row4 = row1.clone();
    row4.shuffle(&mut thread_rng());
    let mut row5 = row4.clone();
    row5.rotate_right(3);
    let mut row6 = row4.clone();
    row6.rotate_right(6);

    let mut row7 = row1.clone();
    row7.shuffle(&mut thread_rng());
    let mut row8 = row7.clone();
    row8.rotate_right(3);
    let mut row9 = row7.clone();
    row9.rotate_right(6);

    let board = vec![row1, row2, row3, row4, row5, row6, row7, row8, row9];
    let board = Board { items: board };
    for i in 1..=9 {
        for j in 1..=9 {
            match board.get_row(i, j) {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
            match board.get_column(i, j) {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
            match board.get_square(i, j) {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
        }
    }
    Ok(board)
}

fn make_tile(mut row: Vec<u8>) -> Vec<Tile> {
    let mut out = vec![];
    for _ in 0..row.len() {
        out.push(Tile::Num(row.pop().unwrap()));
    }
    out
}
