mod board;
mod args;
mod error;
mod board_gen;
mod config;
use std::path::PathBuf;

use board::Board;
use config::Config;
use args::Args;
use clap::Parser;
use csv::Writer;
use error::Error;

#[cfg(test)]
mod tests;

fn main() {
    let args = Args::parse();
    let verbose = args.verbose;
    let result = run(args);
    match result {
        Ok(()) => (),
        Err(e) => {
            if verbose {println!("{}\nLine {}, in {}", e.message, e.line, e.file)}
            else {println!("{}", e.message)}
        }
    }
}

//The reason for a run function is that using return statements can cause the function to return
//prematurely, meaning that code doesn't get increasing nested. Otherwise, this functions nearly
//identical to main
fn run(args: Args) -> Result<(), Error> {
    let config = Config::new(args.verbose, args.config_dir.clone());
    let board = match load_board(args.clone()){
        Ok(b) => b,
        Err(e) => {
            let line = line!()-3;
            let file = file!().to_string();
            let message = match args.verbose {
                true => "Oops! Couldn't load CSV!\n".to_string() + &e.message,
                false => "Oops! Couldn't load CSV!".to_string()
            };
            return Err(Error::new(message, line, file))
        }
    };
    if args.verbose {board.display(); println!()}

    let mut solved_board = match board.solve(args.max_loop) {
        Ok(x) => x,
        Err(e) => return Err(e)
    };

    //You get one nudge to solve. lol.
    if solved_board.1 && args.attempt_solve {
        Board::solve_attempt(&mut solved_board.0);
        solved_board = match solved_board.0.solve(args.max_loop) {
            Ok(x) => x,
            Err(e) => return Err(e)
        };
    }

    if args.verbose {solved_board.0.display()}
    match export_board(solved_board.0, args.output, args.remove, args.verbose, &config.default_output) {
        Ok(()) => (),
        Err(e) => return Err(e)
    };

    Ok(())
}

pub fn load_board(args: Args) -> Result<Board, Error> {
    let path = match args.path.to_str() {
        Some(x) => x.to_owned(),
        None => {
            let line = line!()-3;
            let file = file!().to_string();
            let message = "Failed to parse path".to_string();
            return Err(Error::new(message, line, file))
        }
    };

    let file = match std::fs::File::open(path) {
        Ok(x) => x,
        Err(e) => {
            let line = line!()-3;
            let file = file!().to_string();
            let mut message = e.to_string();
            if args.verbose {message = message + "\n" + &e.kind().to_string()}
            return Err(Error::new(message, line, file))
        }
    };

    let mut csv_reader = csv::ReaderBuilder::new().has_headers(args.contains_header).from_reader(file);
    let mut board = vec![];
    
    for i in csv_reader.deserialize() {
        let record: Vec<String> = match i {
            Ok(x) => x,
            Err(e) => { 
                let line = line!()-4;
                let file = file!().to_string();
                return Err(Error::new(e.to_string(), line, file));
            }
        };
        board.push(record);
    }
    Board::new(board, args.attempt)
}

pub fn export_board(board: Board, path: Option<PathBuf>, rewrite: bool, verbose: bool, default_out: &String) -> Result<(), Error> {
    let path = match path {
        Some(x) => {
            if x.exists() && !rewrite {
                if verbose {println!("File exists, writing elsewhere")}
                let mut p = PathBuf::new();
                p.push(default_out);
                p
            }
            else {x}
        },
        None => {
            let mut p = PathBuf::new();
            p.push(default_out);
            p
        }
    };
    if verbose {println!("Writing to {:?}", &path)}
    let mut out_board: Vec<Vec<String>> = vec![];
    for i in board.items {
        let mut out_row = vec![];
        for j in i {
            let tile_string = match j {
                board::Tile::Num(n) => n.to_string(),
                board::Tile::Non(n) => {
                    let mut tile = "".to_string();
                    for k in n {
                        tile = tile + &k.to_string() + ", "
                    }
                    tile
                },
            };
            out_row.push(tile_string);
        }
        out_board.push(out_row);
    }

    let mut writer = match Writer::from_path(path) {
        Ok(x) => x,
        Err(_) => {
            let line = line!()-3;
            let file = file!().to_owned();
            let message = "Could not write to file".to_string();
            let error = Error::new(message, line, file);
            return Err(error)
        }
    };
    for i in 0..9 {
        match writer.write_record(&out_board[i]) {
            Ok(_) => (),
            Err(_) => {
                let line = line!()-3;
                let file = file!().to_owned();
                let message = "Could not write to file".to_string();
                let error = Error::new(message, line, file);
                return Err(error)
            }
        }
    };
    writer.flush().unwrap();
    Ok(())
}
