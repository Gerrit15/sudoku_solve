//Num for when a tile has a single state, to allow for faster iteration
//Non for when a tile's entropy is not zero
//consider in future using Vec of Tiles
enum Tile {
    Num(u8),
    Non(Vec<u8>)
}

//a simple 2d data structure to hold the sudoku board
pub struct Board {
    items: Vec<Vec<Tile>>
}

impl Board {
    //generates a new Board given a 2d Vec of Strings
    pub fn new(items: Vec<Vec<String>>) -> Board {
        let mut board = vec![];
        for i in items {
            let mut row = vec![];
            for j in i {
                match j.parse::<u8>() {
                    Ok(x) => row.push(Tile::Num(x)),
                    //1, 2, 3 is placeholder
                    Err(_) => row.push(Tile::Non(vec![]))
                }
            }
            board.push(row);
        }
        Board {
            items: board
        }
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
