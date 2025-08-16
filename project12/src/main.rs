use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader};

fn main() {
    let f1 = OpenOptions::new()
        .read(true)
        .open("marks.txt")
        .unwrap();
    let mut lines = vec![];
    let reader = BufReader::new(f1);

    for line in reader.lines(){
        lines.push(line.unwrap());
    } 

    println!("{:?}", lines);
}
