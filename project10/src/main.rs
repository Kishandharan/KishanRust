use std::thread;

fn main() {
    let thread1 = thread::spawn(|| {
        let var1 = 283+23; 
        println!("{}", var1);
    });
    let thread2 = thread::spawn(|| {
        let var2 = 3249575+273; 
        println!("{}", var2);
    });
    let thread3 = thread::spawn(|| {
        let var3 = 4748+2938; 
        println!("{}", var3);
    });
    let thread4 = thread::spawn(|| {
        let var4 = 105+105; 
        println!("{}", var4);
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
}
