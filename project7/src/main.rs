use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

fn main() {
    let mut f1 = OpenOptions::new()
        .read(true)
        .create(true)
        .open("file1.txt")
        .unwrap();
    let reader = BufReader::new(f1);
    let mut lines = vec![];

    for line in reader.lines(){
        lines.push(line.unwrap());
    }

    println!("{:?}", lines);
}
