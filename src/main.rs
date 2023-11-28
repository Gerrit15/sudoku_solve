fn main() {
    let path = "example.csv".to_owned();
    let board = load_board(path);

    for i in board {
        for j in i {
            print!("{}, ", j);
        }
        println!();
    }
}

//this function takes a file system path, and returns a 2d vec of strings if it's a properly formatted
//CSV, using the aptly named CSV crate. This is extremely volitile, and cannot handle improperly
//formatted files
fn load_board(path: String) -> Vec<Vec<String>> {
    let file = std::fs::File::open(path).unwrap();

    //let "has headers" be a command line option
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut board = vec![];
    for i in csv_reader.deserialize() {
        let record: Vec<String> = i.unwrap();
        board.push(record);
    }
    return board
}

//Num for when a tile has a single state, to allow for faster iteration
//Non for when a tile's entropy is not zero
enum Tile {
    Num(u8),
    Non(Vec<u8>)
}

//a simple 2d data structure to hold the sudoku board
struct Board {
    items: Vec<Vec<Tile>>
}
