use std::fs::OpenOptions;
use std::io::Write;

fn main(){    
    let mut f1 = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("f1.txt")
        .unwrap();
        
    f1.write_all("This is DOPE dude!".as_bytes()).unwrap();
}