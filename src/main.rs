mod board;
mod args;
mod error;
use board::Board;
use args::Args;
use clap::Parser;
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
    let board = match load_board(args.clone()){
        Ok(b) => b,
        Err(e) => {
            let line = line!()-3;
            let file = file!().to_string();
            let message = match args.verbose {
                //make sure to account for verbose
                true => "Oops! Couldn't load CSV!\n".to_string() + &e.message,
                false => "Oops! Couldn't load CSV!".to_string()
            };
            //println!("Oops! Couldn't load CSV!");
            return Err(Error::new(message, line, file))
        }
    };
    if args.verbose {board.display()}

    Ok(())
}

pub fn load_board(args: Args) -> Result<Board, Error> {
    //This is give None if it couldn't turn the path into a string
    let path = match args.path.to_str() {
        Some(x) => x.to_owned(),
        None => {
            let line = line!()-3;
            let file = file!().to_string();
            let message = "Failed to parse path".to_string();
            return Err(Error::new(message, line, file))
        }
    };

    //This will Err<t> if the path provided doesn't have a destination
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
    
    //This will Err<t> if the provided file doesn't go well.
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
