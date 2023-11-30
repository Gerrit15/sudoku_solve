mod board;
use std::path::PathBuf;
use board::Board;
use clap::Parser;
#[cfg(test)]
mod tests;

fn main() {
    let args = Args::parse();
    let board = load_board(args);
    match board {
        Ok(b) => b.display(),
        Err(e) => {
            println!("Oops! Something went wrong!\n{}", e);
        }
    }
}

//this function takes a file system path, and (hopefully) returns a 2d vec of strings if it's a properly formatted
//CSV, using the aptly named CSV crate. If something goes wrong, it will return a string with an
//error message.
fn load_board(args: Args) -> Result<Board, String> {

    let path = match args.path.to_str() {
        Some(x) => x.to_owned(),
        None => return Err("Failed to parse path".to_owned())
    };

    let file = match std::fs::File::open(path) {
        Ok(x) => x,
        Err(e) => {
            let mut error_msg = e.to_string();
            if args.verbose {error_msg = error_msg + "\n" + &e.kind().to_string()}
            return Err(error_msg)
        }
    };

    let mut csv_reader = csv::ReaderBuilder::new().has_headers(args.contains_header).from_reader(file);
    let mut board = vec![];
    for i in csv_reader.deserialize() {
        let record: Vec<String> = match i {
            Ok(x) => x,
            Err(e) => { 
               return Err(e.to_string())
            }
        };
        board.push(record);
    }
    Board::new(board)
}

//atn: refactoring Args is not worth it. Parser does not like fields being public
///TODO: command line description
#[derive(Parser)]
struct Args {
    ///Path of the desired Sudoku board
    path: PathBuf,

    ///Does the CSV have a header
    #[arg(short, long)]
    contains_header: bool,

    ///Provides verbose error output
    #[arg(short, long)]
    verbose: bool,

    ///Attempts a little harder to load the board
    #[arg(short, long)]
    attempt: bool
}

//Other potential options:
//  - Creative: tries more to load numbers (ex: cutting leading whitespace)
//  - Strict: does not? Might be redundant
//  - Output: where to put the output of the file
//  - Displaying Each step/Walking through steps? For solving
