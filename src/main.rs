mod board;
use std::path::PathBuf;
use board::Board;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let board = load_board(args).unwrap();
    board.display();
}

//this function takes a file system path, and returns a 2d vec of strings if it's a properly formatted
//CSV, using the aptly named CSV crate. This is extremely volitile, and cannot handle improperly
//formatted files
fn load_board(args: Args) -> Result<Board, String> {

    let path = match args.path.to_str() {
        Some(x) => x.to_owned(),
        None => return Err("Failed to parse path".to_owned())
    };

    //let file = std::fs::File::open(path).unwrap();
    let file = match std::fs::File::open(path) {
        Ok(x) => x,
        Err(x) => return Err(x.to_string())
    };

    //let "has headers" be a command line option
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(args.contains_header).from_reader(file);
    let mut board = vec![];
    for i in csv_reader.deserialize() {
        let record: Vec<String> = i.unwrap();
        board.push(record);
    }
    Ok(Board::new(board))
}

///TODO: command line description
#[derive(Parser)]
struct Args {
    ///Path of the desired Sudoku board
    path: PathBuf,

    ///Does the CSV have a header
    #[arg(short, long)]
    contains_header: bool,

    ///Provides verbose error messages
    #[arg(short, long)]
    verbose: bool
}

//atn: refactoring Args is not worth it. Parser does not like fields being public
//Other potential options:
//  - Creative: tries more to load numbers
//  - Strict: does not? Might be redundant
//  - Output: where to put the output of the file
