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
        Err(e) => println!("{}", e)
    }
}

//The reason for a run function is that using return statements can cause the function to return
//prematurely, meaning that code doesn't get increasing nested. Otherwise, this functions nearly
//identical to main
fn run() -> Result<(), String> {
    let args = Args::parse();
    let board = match Board::load_board(args.clone()){
        Ok(b) => b,
        Err(e) => {
            println!("Oops! Couldn't load CSV!");
            return Err(e)
        }
    };
    if args.verbose {board.display()}

    Ok(())
}
