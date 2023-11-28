fn main() {
    let path = "/home/gerrit/projects/sudoku_solve/example.csv".to_owned();
    let file = std::fs::File::open(path).unwrap();

    //let "has headers" be a command line option
    let mut csv_reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    println!("{:?}", csv_reader.records().next().unwrap().unwrap());
}
