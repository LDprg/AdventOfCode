use regex::Regex;
use std::fs::*;
use std::io::*;

fn main() {
    let file = File::open("./res/input_real.txt").unwrap();

    let mut sum = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        for item in pattern.captures_iter(&line) {
            if (item[1] >= item[3] && item[2] <= item[4])
            || (item[3] >= item[1] && item[4] <= item[2])
            {
                sum += 1;
                println!("found: {}-{}, {}-{}", &item[1], &item[2], &item[3], &item[4]);
            }
        }
    }

    println!("\n{}", sum); 
}
