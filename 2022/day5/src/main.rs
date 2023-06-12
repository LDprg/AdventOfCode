use std::fs::*;
use std::io::*;

fn main() {
    let file = File::open("res/input_test.txt").unwrap();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        println!("{}", line);
    }
}
