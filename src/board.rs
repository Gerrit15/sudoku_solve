//Num for when a tile has a single state, to allow for faster iteration
//Non for when a tile's entropy is not zero
//consider in future using Vec of Tiles
#[derive(PartialEq, Debug)]
pub enum Tile {
    Num(u8),
    Non(Vec<u8>)
}
use super::Args;

//a simple 2d data structure to hold the sudoku board
#[derive(PartialEq, Debug)]
pub struct Board {
    pub items: Vec<Vec<Tile>>
}

impl Board {
    //this function takes a file system path, and (hopefully) returns a 2d vec of strings if it's a properly formatted
    //CSV, using the aptly named CSV crate. If something goes wrong, it will return a string with an error message.
    pub fn load_board(args: Args) -> Result<Board, String> {

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
    //generates a new Board given a 2d Vec of Strings
    pub fn new(items: Vec<Vec<String>>, attempt: bool) -> Result<Board, String> {
        let mut board = vec![];
        for i in items {
            let mut row = vec![];
            for j in i {
                match j.parse::<u8>() {
                    Ok(x) => {
                        if x > 9 || x == 0 {return Err("Board contains invalid number".to_string())}
                        row.push(Tile::Num(x))
                    },
                    Err(_) => {
                        if attempt {
                            let mut found = false;
                            for k in 1..10 {
                                if j.contains(&k.to_string()) && !found {
                                    row.push(Tile::Num(k));
                                    found = true;
                                }
                            }
                            if !found {row.push(Tile::Non(vec![]))}
                        } else {
                            row.push(Tile::Non(vec![]))
                        }
                    }
                }
            }
            board.push(row);
        }
        if board.len() != 9 {return Err("Board is not 9 rows".to_string())}
        //It is given by the CSV crate that every row is the same length
        if board[0].len() != 9 {return Err("Board is not 9 columns".to_string())}

        Ok(Board {
            items: board
        })
    }

    //iterates through a reference to the board's tiles, printing them out
    pub fn display(&self) {
        for i in &self.items {
            for j in i {
                match j {
                    Tile::Num(x) => print!("{x}, "),
                    Tile::Non(x) => print!("{:?}, ", x)
                }
            }
            println!();
        }
    }
}
