use std::fs::*;
use std::io::*;

fn main() {
    let file = File::open("./res/input_real.txt").unwrap();

    let mut sum = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let data = line.split(&[',', '-']).map(|str| {
            str.parse::<u16>().unwrap()
        }).collect::<Vec<_>>();
            
        let range1  = &data[0]..=&data[1];
        let range2 = &data[2]..=&data[3];

        if (range1.contains(range2.start()) && range1.contains(range2.end()))
            || (range2.contains(range1.start()) && range2.contains(range1.end()))
        {
            sum += 1;
            println!("Found: {}-{}, {}-{}", range1.start(), range1.end(), range2.start(), range2.end());
        }
    }

    println!("\n{}", sum);
}
