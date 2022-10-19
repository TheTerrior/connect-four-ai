mod neuralnet;

use std::{fs, io::Error};

fn main() {
    let target: String = String::from("tempdatabase.gn");
    let result = fs::read(target).unwrap();
    println!("hi");
}