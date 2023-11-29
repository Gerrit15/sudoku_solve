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
    pub fn new(items: Vec<Vec<String>>) -> Result<Board, String> {
        let mut board = vec![];
        for i in items {
            let mut row = vec![];
            for j in i {
                match j.parse::<u8>() {
                    Ok(x) => {
                        if x > 9 {return Err("Board contains number greater then 9".to_string())}
                        row.push(Tile::Num(x))
                    },
                    Err(_) => row.push(Tile::Non(vec![]))
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
