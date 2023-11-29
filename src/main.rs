mod board;
use board::Board;

fn main() {
    let path = "example.csv".to_owned();
    let board = load_board(path);
    board.display();
}

//this function takes a file system path, and returns a 2d vec of strings if it's a properly formatted
//CSV, using the aptly named CSV crate. This is extremely volitile, and cannot handle improperly
//formatted files
pub fn load_board(path: String) -> Board {
    let file = std::fs::File::open(path).unwrap();

    //let "has headers" be a command line option
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut board = vec![];
    for i in csv_reader.deserialize() {
        let record: Vec<String> = i.unwrap();
        board.push(record);
    }
    Board::new(board)
}

