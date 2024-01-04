use super::{Args, Board, board::Tile, load_board, board_gen::{gen_board, poke_holes}};
use std::path::PathBuf;

#[test]
fn base_load() {
    let args = arg_gen("test_csvs/base.csv", false, false);
    let board = load_board(args);
    assert!(matches!(board, Ok(_)));
}

#[test]
//non as in Tile:Non(Vec<u8>)
fn base_non() {
    use Tile::*;
    let args = arg_gen("test_csvs/non.csv", false, false);
    let board = load_board(args);
    let b2 = Board {
        //This is to make sure that the Non is placed in the right spot
        items: vec![
            vec![Num(4),Num(3),Num(5),Num(2),Num(6),Num(9),Num(7),Num(8),Num(1)],
            vec![Num(6),Num(8),Num(2),Num(5),Num(7),Num(1),Non(vec![]),Num(9),Num(3)],
            vec![Num(1),Num(9),Num(7),Num(8),Num(3),Num(4),Num(5),Num(6),Num(2)],
            vec![Num(8),Non(vec![]),Num(6),Num(1),Num(9),Num(5),Num(3),Num(4),Num(7)],
            vec![Num(3),Num(7),Num(4),Num(6),Num(8),Num(2),Num(9),Num(1),Num(5)],
            vec![Num(9),Num(5),Num(1),Num(7),Num(4),Num(3),Num(6),Num(2),Num(8)],
            vec![Num(5),Non(vec![]),Num(9),Num(3),Num(2),Num(6),Num(8),Num(7),Num(4)],
            vec![Num(2),Num(4),Num(8),Num(9),Num(5),Num(7),Num(1),Num(3),Num(6)],
            vec![Num(7),Num(6),Num(3),Num(4),Num(1),Num(8),Non(vec![]),Num(5),Num(9)],
        ]
    };
    assert_eq!(board.unwrap(), b2);
}

#[test]
fn header() {
    let args = arg_gen("test_csvs/header.csv", true, false);
    let board = load_board(args);
    assert!(matches!(board, Ok(_)));
}

#[test]
fn too_tall() {
    let args = arg_gen("test_csvs/long.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Board is not 9 rows")
    }
}

#[test]
fn too_short() {
    let args = arg_gen("test_csvs/short.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Board is not 9 rows")
    }
}

#[test]
fn too_wide() {
    let args = arg_gen("test_csvs/wide.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Board is not 9 columns")
    }
}

#[test]
fn too_narrow() {
    let args = arg_gen("test_csvs/narrow.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Board is not 9 columns")
    }
}

#[test]
fn out_of_bounds() {
    let args = arg_gen("test_csvs/out_of_bound.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Board contains invalid number")
    }
}

#[test]
fn non_uniform() {
    let args = arg_gen("test_csvs/non_uniform.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "CSV error: record 1 (line: 2, byte: 18): found record with 7 fields, but the previous record has 9 fields")
    }
}

#[test]
//This is because it gets in a deadlock when all 81 tiles are empty
fn empty() {
    let args = arg_gen("test_csvs/empty.csv", false, false);
    match load_board(args) {
        Ok(_) => panic!("returned valid board"),
        Err(b) => assert_eq!(&b.message, "Board is entirely empty")
    }
}

#[test]
fn attempt() {
    use Tile::*;
    let args = arg_gen("test_csvs/attempt.csv", false, true);
    let board = load_board(args);
    let b2 = Board {
        items: vec![
            vec![Num(4),Num(3),Num(5),Num(2),Num(6),Num(9),Num(7),Num(8),Num(1)],
            vec![Num(6),Num(8),Num(2),Num(5),Num(7),Num(1),Num(4),Num(9),Num(3)],
            vec![Num(1),Num(9),Num(7),Num(8),Num(3),Num(4),Num(5),Num(6),Num(2)],
            vec![Num(8),Num(2),Num(6),Num(1),Num(9),Num(5),Num(3),Num(4),Num(7)],
            vec![Num(3),Num(7),Num(4),Num(6),Num(8),Num(2),Num(9),Num(1),Num(5)],
            vec![Num(9),Num(5),Num(1),Num(7),Num(4),Num(3),Num(6),Num(2),Num(8)],
            vec![Num(5),Non(vec![]),Num(9),Num(3),Num(2),Num(6),Num(8),Num(7),Num(4)],
            vec![Num(2),Num(4),Num(8),Num(9),Num(5),Num(7),Num(1),Num(3),Num(6)],
            vec![Num(7),Num(6),Num(3),Num(4),Num(1),Num(8),Non(vec![]),Num(5),Num(9)],
        ]
    };
    assert_eq!(board.unwrap(), b2);
}

#[test]
fn invalid_path() {
    let args = arg_gen("test_csvs/uh-oh.missing", false, false);
     match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "No such file or directory (os error 2)")
    }
}

#[test]
fn row_dupes() {
    let args = arg_gen("test_csvs/row_dupe.csv", false, false);
     match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Row contains duplicates")
    }
}

#[test]
fn column_dupes() {
    let args = arg_gen("test_csvs/column_dupe.csv", false, false);
     match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Column contains duplicates")
    }
}

#[test]
fn square_dupes() {
     let args = arg_gen("test_csvs/square_dupe.csv", false, false);
     match load_board(args) {
        Ok(_) => panic!("returned a valid board"),
        Err(b) => assert_eq!(&b.message, "Square contains duplicates")
    }
}

#[test]
fn collapse_single() {
    let args = arg_gen("test_csvs/collapse_single.csv", false, false);
    let board = match load_board(args) {
        Ok(b) => b,
        Err(_) => panic!("Returned bad board")
    };
    assert_eq!(board.collapse_tile(8, 1), Ok(Tile::Non(vec![1, 2, 8])));
}

#[test]
fn collapse_board() {
    let args = arg_gen("test_csvs/collapse_single.csv", false, false);
    let board = match load_board(args) {
        Ok(b) => b,
        Err(_) => panic!("Returned bad board")
    };
    let board = match board.collapse_board() {
        Ok(b) => b,
        Err(_) => panic!("Could not collapse board")
    };
    use Tile::*;
    let b2 = Board {
        //This is to make sure that the Non is placed in the right spot
        items: vec![
            vec![Num(4),Num(3),Num(5),Num(2),Num(6),Num(9),Num(7),Non(vec![1,8]),Num(1)],
            vec![Num(6),Num(8),Num(2),Num(5),Num(7),Num(1),Num(4),Num(9),Num(3)],
            vec![Num(1),Num(9),Num(7),Num(8),Num(3),Num(4),Num(5),Num(6),Num(2)],
            vec![Num(8),Num(2),Num(6),Num(1),Num(9),Num(5),Num(3),Num(4),Num(7)],
            vec![Num(3),Num(7),Num(4),Num(6),Num(8),Num(2),Num(9),Num(1),Num(5)],
            vec![Num(9),Num(5),Num(1),Num(7),Num(4),Num(3),Num(6),Num(2),Num(8)],
            vec![Num(5),Num(1),Num(9),Num(3),Num(2),Num(6),Num(8),Num(7),Num(4)],
            vec![Num(2),Num(4),Num(8),Num(9),Num(5),Num(7),Num(1),Num(3),Num(6)],
            vec![Num(7),Num(6),Num(3),Num(4),Num(1),Num(8),Num(2),Num(5),Num(9)],
        ]
    };
    assert_eq!(board, b2);
}

use proptest::prelude::*;
proptest! {
    //This is twelve because it seems sit breaks down at 13 ¯\_(ツ)_/¯
    #[test]
    fn twelve_test(_ in 1u32..100000) {
        let b = match gen_board() {
            Ok(b) => b,
            Err(_) => panic!("Couldn't board gen")
        };
        let holey = match poke_holes(&b, 12).solve(None) {
            Ok(n) => n.0,
            Err(_) => panic!("Couldn't solve")
        };
        assert_eq!(b, holey)
    }

    //This test, and the following 2, are based on the assumption that in any sudoku board,
    //The sum of the tiles must be equal to or greater then 45 (9+8+7..+1). The "greater then"
    //Comes from Tile::Non, which also gets summed up.
    #[test]
    fn row_sum(_ in 1u32..100000) {
        let b = match gen_board() {
            Ok(b) => b,
            Err(_) => panic!("Couldn't board gen")
        };

        let holey = poke_holes(&b, rand::thread_rng().gen_range(1..=80)).solve(None);
        let mut row_sum = vec![];
        for i in holey.unwrap().0.items {
            let mut n = 0;
            for j in i {
                match j {
                    Tile::Num(val) => n += val as u32,
                    Tile::Non(vec) => {n += vec.iter().sum::<u8>() as u32}
                };
            }
            row_sum.push(n);
        }
        for i in row_sum {
            //45 is 9+8+7..+1
            assert!(i >= 45)
        }
    }

    #[test]
    fn col_sum(_ in 1u32..100000) {
        let b = match gen_board() {
            Ok(b) => b,
            Err(_) => panic!("Couldn't board gen")
        };

        let holey = poke_holes(&b, rand::thread_rng().gen_range(1..=80)).solve(None).unwrap();
        let mut col_sum = vec![];
        for i in 0..9 {
            let mut n = 0;
            for j in 0..9 {
                match &holey.0.items[j][i] {
                    Tile::Num(val) => n += val.clone() as u32,
                    Tile::Non(vec) => n += vec.iter().sum::<u8>() as u32
                }
            }
            col_sum.push(n);
        }
        for i in col_sum {
            assert!(i >= 45)
        }
    }

    fn square_sum(_ in 1u32..100000) {
        let b = match gen_board() {
            Ok(b) => b,
            Err(_) => panic!("Couldn't board gen")
        };

        let holey = poke_holes(&b, rand::thread_rng().gen_range(1..=80)).solve(None).unwrap();
        let mut square_sum: Vec<u32> = vec![];
        //Adapted from the get_square() fn in board.rs
        let tile_indeces = [1, 4, 7];
        for centerx in tile_indeces {
            for centery in  tile_indeces {
                let mut n = 0;
                for i in centerx-1..=centerx+1 {
                    for j in centery-1..=centery+1 {
                        match &holey.0.items[j][i] {
                            Tile::Num(val) => n += val.clone() as u32,
                            Tile::Non(vec) => n += vec.iter().sum::<u8>() as u32
                        }
                    }
                }
                square_sum.push(n)
            }
        }
        for i in square_sum {
            assert!(i >= 45)
        }
    }
}

//A simple function to generate example cmd line arguements, to avoid repetitive code in tests.
fn arg_gen(path: &str, contains_header:bool, attempt: bool) -> Args {
    Args {
        path: PathBuf::from(path),
        contains_header,
        verbose: false,
        attempt,
        remove: false,
        output: None
    }
}
