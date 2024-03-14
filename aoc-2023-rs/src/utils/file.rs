use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}

pub fn read_file_buffered(path: &Path) -> BufReader<File> {
    let file = File::open(path).expect("Should have been able to read the file");
    BufReader::new(file)
}
