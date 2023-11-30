use super::{Args, Board, board::Tile, load_board};
use std::path::PathBuf;

//To make sure a basic board is loaded in properly
#[test]
fn base_load() {
    use Tile::*;
    let args = Args {
        path: PathBuf::from("test_csvs/base.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args).unwrap();
    let b2 = Board {
        items: vec![
            vec![Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1)],
            vec![Num(2),Num(2),Num(2),Num(2),Num(2),Num(2),Num(2),Num(2),Num(2)],
            vec![Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3)],
            vec![Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4)],
            vec![Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5)],
            vec![Num(6),Num(6),Num(6),Num(6),Num(6),Num(6),Num(6),Num(6),Num(6)],
            vec![Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7)],
            vec![Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8)],
            vec![Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9)]
        ]
    };
    assert_eq!(board, b2);
}

//To confirm that a board with (proper) holes can be loaded properly
#[test]
fn base_non() {
    use Tile::*;
    let args = Args {
        path: PathBuf::from("test_csvs/non.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args).unwrap();
    let b2 = Board {
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
    use Tile::*;
    let args = Args {
        path: PathBuf::from("test_csvs/header.csv"),
        contains_header: true,
        verbose: true
    };
    let board = load_board(args).unwrap();
    let b2 = Board {
        items: vec![
            vec![Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1),Num(1)],
            vec![Num(2),Num(2),Num(2),Num(2),Num(2),Num(2),Num(2),Num(2),Num(2)],
            vec![Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3),Num(3)],
            vec![Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4),Num(4)],
            vec![Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5),Num(5)],
            vec![Num(6),Num(6),Num(6),Num(6),Num(6),Num(6),Num(6),Num(6),Num(6)],
            vec![Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7),Num(7)],
            vec![Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8),Num(8)],
            vec![Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9),Num(9)]
        ]
    };
    assert_eq!(board, b2);
}

//To confirm that a board that's too tall (too many rows) isn't loaded in
#[test]
fn too_tall() {
     let args = Args {
        path: PathBuf::from("test_csvs/long.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 rows".to_string()))
}

//To confirm that a board that's too short (not enough rows) isn't loaded in
#[test]
fn too_short() {
     let args = Args {
        path: PathBuf::from("test_csvs/short.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 rows".to_string()))
}

//To confirm that a board that's too wide (too many columns) isn't loaded
#[test]
fn too_wide() {
     let args = Args {
        path: PathBuf::from("test_csvs/wide.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 columns".to_string()))
}

//To confirm that a board that's too narrow (not enough columns) isn't loaded
#[test]
fn too_narrow() {
     let args = Args {
        path: PathBuf::from("test_csvs/narrow.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args);
    assert_eq!(board, Err("Board is not 9 columns".to_string()))
}

//To confirm that a board that contains numbers greater then 9 isn't loaded
#[test]
fn out_of_bounds() {
     let args = Args {
        path: PathBuf::from("test_csvs/out_of_bound.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args);
    assert_eq!(board, Err("Board contains number greater then 9".to_string()))
}

//To confirm that a board with non uniform width isn't loaded
#[test]
fn non_uniform() {
     let args = Args {
        path: PathBuf::from("test_csvs/non_uniform.csv"),
        contains_header: false,
        verbose: false
    };
    let board = load_board(args);
    assert_eq!(board, Err("CSV error: record 1 (line: 2, byte: 18): found record with 7 fields, but the previous record has 9 fields".to_string()))
}
