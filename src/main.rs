//plan: Serde example.csv in -> csv crate, print it all out
//plan isn't so hot, dump Serde? dunno.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::fs::File;
use clap::Parser;
use std::path::PathBuf;

fn main () {
    let args = Args::parse();
    let path = args.path.to_str().unwrap().to_owned();
    let file = File::open(path).unwrap();
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

enum Tile {
    Num(u8),
    Non(Vec<u8>)
}

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
