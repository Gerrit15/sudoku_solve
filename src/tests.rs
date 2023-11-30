use super::{Args, Board, board::Tile, load_board};
use std::path::PathBuf;

//To make sure a basic board is loaded in properly
#[test]
fn base_load() {
    let args = arg_gen("test_csvs/base.csv", false);
    let board = load_board(args);
    assert!(matches!(board, Ok(_)));
}

//To confirm that a board with (proper) holes can be loaded properly
#[test]
fn base_non() {
    use Tile::*;
    let args = arg_gen("test_csvs/non.csv", false);
    let board = load_board(args).unwrap();
    let b2 = Board {
        //This is to make sure that the Non is placed in the right spot
        items: vec![
            vec![Num(1),Non(vec![]),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1)],
            vec![Num(2),Num(2),Num(2),Non(vec![]),Num(2),Num(2),Num(2),Num(2),Num(2)],
            vec![Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3)],
            vec![Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4)],
            vec![Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5)],
            vec![Num(6),Num(6),Num(6),Num(6),Num(6),Non(vec![]),Non(vec![]),Num(6),Num(6)],
            vec![Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7)],
            vec![Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8)],
            vec![Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9)],
        ]
    };
    assert_eq!(board, b2);
}

//To confirm files with headers can be loaded
#[test]
fn header() {
    let args = arg_gen("test_csvs/header.csv", true);
    let board = load_board(args);
    assert!(matches!(board, Ok(_)));
}

//To confirm that a board that's too tall (too many rows) isn't loaded in
#[test]
fn too_tall() {
    let args = arg_gen("test_csvs/long.csv", false);
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 rows".to_string()))
}

//To confirm that a board that's too short (not enough rows) isn't loaded in
#[test]
fn too_short() {
    let args = arg_gen("test_csvs/short.csv", false);
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 rows".to_string()))
}

//To confirm that a board that's too wide (too many columns) isn't loaded
#[test]
fn too_wide() {
    let args = arg_gen("test_csvs/wide.csv", false);
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 columns".to_string()))
}

//To confirm that a board that's too narrow (not enough columns) isn't loaded
#[test]
fn too_narrow() {
    let args = arg_gen("test_csvs/narrow.csv", false);
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 columns".to_string()))
}

//To confirm that a board that contains numbers greater then 9 isn't loaded
#[test]
fn out_of_bounds() {
    let args = arg_gen("test_csvs/out_of_bound.csv", false);
    let board = load_board(args);
    assert_eq!(board, Err("Board contains number greater then 9".to_string()))
}

//To confirm that a board with non uniform width isn't loaded
#[test]
fn non_uniform() {
    let args = arg_gen("test_csvs/non_uniform.csv", false);
    let board = load_board(args);
    assert_eq!(board, Err("CSV error: record 1 (line: 2, byte: 18): found record with 7 fields, but the previous record has 9 fields".to_string()))
}

//A simple function to generate example cmd line arguements
fn arg_gen(path: &str, header:bool) -> Args {
    Args {
        path: PathBuf::from(path),
        contains_header: header,
        verbose: false,
        attempt: false
    }
}
