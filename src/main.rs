use std::io;

fn main() {
    println!("Hello!");
    println!("続けるには Enter を押してください...");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}