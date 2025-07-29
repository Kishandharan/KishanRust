use std::io::{Write, BufWriter};
use std::fs::OpenOptions;

fn main() {
    let f1 = OpenOptions::new()
        .create(true)
        .write(true)
        .open("example.txt")
        .unwrap();
    let mut writer1 = BufWriter::new(f1);
    for _ in 1..=10{
        writer1.write_all(b"Trello\n").unwrap();
    }
    writer1.flush().unwrap();
}
