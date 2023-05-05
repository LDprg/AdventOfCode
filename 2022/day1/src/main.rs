use std::fs::*;
use std::io::*;

fn main() {
    let file = File::open("./res/input_test.txt").unwrap();
    let mut list: Vec<i32> = Vec::new();
    list.push(0);

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        match line.parse::<i32>() {
            Ok(num) => {
                if let Some(last) = list.last_mut() {
                    *last += num;
                }
            }
            Err(_) => {
                list.push(0);
            }
        }
    }

    println!("{}", list.iter().max().unwrap());
}
