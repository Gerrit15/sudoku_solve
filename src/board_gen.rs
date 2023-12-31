//Based off of the algorithm outlined in this post:
//https://gamedev.stackexchange.com/questions/56149/how-can-i-generate-sudoku-puzzles
//Specifically the second solution, at time of recording.
//
//Not sure if it makes every possible board, but it works well enough for general generation

use super::Board;
use super::board::Tile;
use super::Error;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

#[allow(dead_code)]
pub fn gen_board() -> Result<Board, Error> {
    let mut row1 = make_tile((1..=9).collect::<Vec<u8>>());

    row1.shuffle(&mut thread_rng());
    let mut row2 = row1.clone();
    row2.rotate_right(3);
    let mut row3 = row1.clone();
    row3.rotate_right(6);

    let mut row4 = row1.clone();
    row4.rotate_right(1);
    let mut row5 = row4.clone();
    row5.rotate_right(3);
    let mut row6 = row4.clone();
    row6.rotate_right(6);

    let mut row7 = row1.clone();
    row7.rotate_right(2);
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

#[allow(dead_code)]
//returns a board with holes poked in it
pub fn poke_holes(board: &Board, num_holes: u32) -> Board {
    let mut holey = board.clone();
    let mut rng = thread_rng();
    for _ in 0..=num_holes {
        loop {
            let randx: usize = rng.gen_range(0..9);
            let randy: usize = rng.gen_range(0..9);
            let has_value = match holey.items[randy][randx] {
                Tile::Num(_) => true,
                Tile::Non(_) => false
            };
            if has_value {
                holey.items[randy][randx] = Tile::Non(vec![]);
                break
            }
        }
    }
    holey
}

fn make_tile(mut row: Vec<u8>) -> Vec<Tile> {
    let mut out = vec![];
    for _ in 0..row.len() {
        out.push(Tile::Num(row.pop().unwrap()));
    }
    out
}
