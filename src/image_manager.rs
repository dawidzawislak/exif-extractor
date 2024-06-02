use std::io::Read;
use std::fs::File;

pub fn open_image(path: String) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}