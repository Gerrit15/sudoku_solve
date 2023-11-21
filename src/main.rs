//plan: Serde example.csv in -> csv crate, print it all out
//plan isn't so hot, dump Serde? dunno.
use std::fs::File;

struct Board{
    items: Vec<Vec<Tile>>
}

#[allow(dead_code)]
enum Tile {
    Num(u8),
    Non(Vec<u8>)
}

fn main () {
    let file = File::open("/home/gerrit/projects/sudoku_solve/example.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
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
    for i in board.items {
        for j in i {
            match j {
                Tile::Num(x) => print!("{x}, "),
                Tile::Non(x) => print!("{:?}, ", x)
            }
        }
        println!("");
    }
}
