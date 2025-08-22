use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader};

fn split(inpstr: &str, spltref: &str){
    let mut output_vec : Vec<String> = vec![];
    let mut temp_string : String;

    for char in inpstr.chars(){
        if char != spltref{
            temp_string.push_str(char);
        }else{
            output_vec.push(temp_string);
            temp_string.clear();
        }
    }
}

fn main() {
    let inp_file : _ = OpenOptions::new()
        .read(true)
        .open("marks.txt")
        .unwrap();

    let mut marks1 : Vec<_> = vec![];
    let mut marks2 : Vec<_> = vec![];
    let mut marks3 : Vec<_> = vec![];
    let mut marks4 : Vec<_> = vec![];
    let mut marks5 : Vec<_> = vec![];
    let file_reader : BufReader<_> = BufReader::new(inp_file);

    for line in file_reader.lines(){
        let unwrapped_line = line.unwrap();
    }

}
