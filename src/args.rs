use std::path::PathBuf;
use clap::Parser;

//Note that the triple slash is for the clap crate, it's what will show up with -h

///TODO: command line description
#[derive(Parser, Clone)]
pub struct Args {
    ///Path of the desired Sudoku board
    pub path: PathBuf,

    ///Does the CSV have a header
    #[arg(short, long)]
    pub contains_header: bool,

    ///Provides verbose error output
    #[arg(short, long)]
    pub verbose: bool,

    ///Attempts a little harder to interpret numbers in csv
    #[arg(short, long)]
    pub attempt: bool,

    ///Whether or not to override "remove" the file it points to
    #[arg(short, long)]
    pub remove: bool,

    /// Where to send the output
    #[arg(short, long, value_name="FILE")]
    pub output: Option<PathBuf>
}

//Other potential options:
//  - Output: where to put the output of the file
//  - Displaying Each step/Walking through steps? For solving
//      - In the form of a video or gif? could be a fun stretch goal
//      - animation in terminal?
//  - What do do in the event of a branching pathway?
//  - Generate a board to use?
//  - Config?
//
//Remember to put more chances for verbose output
