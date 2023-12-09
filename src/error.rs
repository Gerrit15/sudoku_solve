#[derive(PartialEq, Debug)]
pub struct Error {
    pub message: String,
    pub line: u32,
    pub file: String,
}

impl Error {
    pub fn new(message: String, line: u32, file: String) -> Error {
        Error {
            message,
            line,
            file
        }
    }
}
