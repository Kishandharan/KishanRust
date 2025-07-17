use std::fs::OpenOptions;
//use std::io::Write;
use std::io::Read;

fn main(){    
    let mut f1 = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("f1.txt")
        .unwrap();

    let mut file_contents : String = String::new();
    //f1.write_all("This is DOPE dude!".as_bytes()).unwrap();
    f1.read_to_string(&mut file_contents).unwrap();
    println!("{}", file_contents)
}