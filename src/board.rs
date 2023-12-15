use super::Error;

#[derive(PartialEq, Debug)]
pub enum Tile {
    Num(u8),
    Non(Vec<u8>)
}

#[derive(PartialEq, Debug)]
pub struct Board {
    pub items: Vec<Vec<Tile>>
}

impl Board {
    pub fn new(items: Vec<Vec<String>>, attempt: bool) -> Result<Board, Error> {
        let mut board = vec![];
        for i in items {
            let mut row = vec![];
            for j in i {
                match j.parse::<u8>() {
                    Ok(x) => {
                        if x > 9 || x == 0 {
                            let line = line!()-1;
                            let file = file!().to_string();
                            let message = "Board contains invalid number".to_string();
                            return Err(Error::new(message, line, file))
                        }
                        row.push(Tile::Num(x))
                    },
                    Err(_) => {
                        if attempt {
                            let mut found = false;
                            //If it needs to attempt a little harder, will try to find the first
                            //number between 1 and 9 in each item, for example "thisis923lalala"
                            //will become 9. Note that 48410 will become 1, not 4.
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
        if board.len() != 9 {
            let line = line!()-1;
            let file = file!().to_string();
            let message = "Board is not 9 rows".to_string();
            return Err(Error::new(message, line, file))
        }
        
        //It is given by the CSV crate that every row is the same length, so we just have to check the first row
        if board[0].len() != 9 {
            let line = line!()-1;
            let file = file!().to_string();
            let message = "Board is not 9 columns".to_string();
            return Err(Error::new(message, line, file))
        }

        let board = Board {
            items: board
        };

        for i in 1..10 {
            match board.get_row(1, i) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
            match board.get_column(i, 1) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }

        for i in [1,5,9] {
            for j in [1,5,9] {
                match board.get_column(i, j) {
                    Ok(_) => (),
                    Err(e) => return Err(e)
                }
            }
        }

        Ok(board)
    }

    pub fn get_row(&self, x1: usize, y1: usize) -> Result<Vec<u8>, Error> {
        if x1 < 1 || x1 > 9 || y1 < 1 || y1 > 9 {
            let line = line!()-1;
            let file = file!().to_string();
            return Err(Error::new("Index is out of bounds".to_owned(), line, file))
        }
        let x = x1 - 1;
        let y = y1 - 1;
        let mut row = vec![];
        for i in 0..9 {
            if i != x {
                match self.items[y][i] {
                    Tile::Num(n) => {
                        //println!("VEC: {:?}, TRYING TO PUSH: {n}", &row);
                        if !row.contains(&n) && (Tile::Num(n) != self.items[y][x]) {row.push(n)}
                        else {
                            let line = line!()-2;
                            let file = file!().to_string();
                            return Err(Error::new("Row contains duplicates".to_string(), line, file))
                        }
                    },
                    Tile::Non(_) => ()
                }
            }
        }
        Ok(row)
    }

    pub fn get_column(&self, x1: usize, y1: usize) -> Result<Vec<u8>, Error> {
        if x1 < 1 || x1 > 9 || y1 < 1 || y1 > 9 {
            let line = line!()-1;
            let file = file!().to_string();
            return Err(Error::new("Index is out of bounds".to_owned(), line, file))
        }
        let x = x1 - 1;
        let y = y1 - 1;
        let mut row = vec![];
        for i in 1..9 {
            if i != y {
                match self.items[i][x] {
                    Tile::Num(n) => {
                        if !row.contains(&n) && (Tile::Num(n) != self.items[y][x]) {row.push(n)}
                        else {
                            let line = line!()-2;
                            let file = file!().to_owned();
                            return Err(Error::new("Column contains duplicates".to_string(), line, file))
                        }
                    },
                    Tile::Non(_) => ()
                }
            }
        }

        Ok(row)
    }

    pub fn get_square(&self, x1:usize, y1: usize) -> Result<Vec<u8>, Error> {
        if x1 < 1 || x1 > 9 || y1 < 1 || y1 > 9 {
            let line = line!()-1;
            let file = file!().to_string();
            return Err(Error::new("Index is out of bounds".to_owned(), line, file))
        }
        let x = x1 - 1;
        let y = y1 - 1;

        //So we take the human position of the tile (ex 1,1 is the top and leftmost tile), and 
        //perform the inner operation: ceil(n/3) to get the "index" of the square (ex the bottom
        //rightmost square is (3, 3), then the outer operation: n*3-2 is performed to get the computer
        //position of the middle tile in the square
        let squarex = ((x1 as f64 /3 as f64).ceil() as usize) * 3 - 2;
        let squarey = ((y1 as f64 /3 as f64).ceil() as usize) * 3 - 2;
        let mut square = vec![];

        for i in (squarex-1)..(squarex+2) {
            for j in (squarey-1)..(squarey+2) {
                match self.items[j][i] {
                    Tile::Num(n) => {
                        if !(j==y && i == x) {
                            if !square.contains(&n) && Tile::Num(n) != self.items[y][x] {square.push(n)}
                            else {
                                let line = line!()-2;
                                let file = file!().to_owned();
                                return Err(Error::new("Square contains duplicates".to_string(), line, file))
                            }
                        }
                    },
                    Tile::Non(_) => ()
                }
            }
        }

        Ok(square)
    }

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
