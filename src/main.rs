mod board;
mod args;
use board::Board;
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
