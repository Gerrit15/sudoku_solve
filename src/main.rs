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
    let result = run();
    match result {
        Ok(()) => (),
        Err(e) => println!("{}", e.message)
    }
}

//The reason for a run function is that using return statements can cause the function to return
//prematurely, meaning that code doesn't get increasing nested. Otherwise, this functions nearly
//identical to main
fn run() -> Result<(), Error> {
    let args = Args::parse();
    let board = match Board::load_board(args.clone()){
        Ok(b) => b,
        Err(e) => {
            let line = line!()-3;
            let file = file!().to_string();
            let message = match args.verbose {
                true => "Oops! Couldn't load CSV!\n".to_string() + &e,
                false => "Oops! Couldn't load CSV!".to_string()
            };
            //println!("Oops! Couldn't load CSV!");
            return Err(Error::new(message, line, file))
        }
    };
    if args.verbose {board.display()}

    Ok(())
}
