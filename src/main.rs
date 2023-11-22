// Thoughts: 
//  - errors are not really handled much
//      - that includes making sure the file and format is correct
//      - that includes clap parameters
//  - serde is gonna take some work
//  - managing column and row groups shouldn't be difficult, but the square
//    groups is going to require finesse

use std::fs::File;
use std::path::PathBuf;
use clap::Parser;

fn main () {
    let args = Args::parse();

    let path = args.path.to_str().unwrap().to_owned();
    let file = File::open(path).unwrap();

    //load in a CSV, use crate to unwrap it, pull out rows, push them into items at Tiles
    let mut rdr = csv::ReaderBuilder::new().has_headers(args.contains_header).from_reader(file);
    let mut items = vec![];
    for result in rdr.deserialize() {
        let record: Vec<String> = result.unwrap();
        let mut row = vec![];
        for i in record {
            match i.parse::<u8>() {
                Ok(x) => row.push(Tile::Num(x)),
                _ => row.push(Tile::Non(vec![1, 2, 3]))
            }
        }
        items.push(row);
    }

    let board = Board{items};
    let mut ok = true;
    if board.items.len() != 9 {ok = false}
    for i in &board.items {
        if i.len() != 9 {ok = false}
    };

    println!("Is board correct? {}", ok);
    board.display();
}

struct Board{
    items: Vec<Vec<Tile>>
}
impl Board {
    fn display(&self) {
        for i in &self.items {
            for j in i {
                match j {
                    Tile::Num(x) => print!("{x}, "),
                    Tile::Non(x) => print!("{:?}, ", x)
                }
            }
            println!("");
        }
    }
}

//This allows each tile to either be in a single state or ... more
enum Tile {
    Num(u8),
    Non(Vec<u8>)
}

//Clap arguements. 

///TODO: description
#[derive(Parser)]
struct Args {
    ///Directory of the Sudoku board
    path: PathBuf,
    //
    ///Does the CSV have a header?
    #[arg(short, long)]
    contains_header: bool
}
