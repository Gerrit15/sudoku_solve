mod board;
use board::Board;
mod args;
use args::Args;
use clap::Parser;

#[cfg(test)]
mod tests;

fn main() {
    let result = run();
    match result {
        Ok(()) => (),
        Err(e) => println!("{}", e)
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let board = match load_board(args.clone()){
        Ok(b) => b,
        Err(e) => {
            println!("Oops! Couldn't load CSV!");
            return Err(e)
        }
    };
    if args.verbose {board.display()}

    Ok(())
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
    Board::new(board, args.attempt)
}
