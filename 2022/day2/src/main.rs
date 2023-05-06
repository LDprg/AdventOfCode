use std::fs::*;
use std::io::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choose {
    Rock,
    Paper,
    Scissors,
}

impl Choose {
    fn from_p1(s: &char) -> Option<Choose> {
        match s {
            'A' => Some(Choose::Rock),
            'B' => Some(Choose::Paper),
            'C' => Some(Choose::Scissors),
            _ => None,
        }
    }

    fn from_p2(s: &char) -> Option<Choose> {
        match s {
            'X' => Some(Choose::Rock),
            'Y' => Some(Choose::Paper),
            'Z' => Some(Choose::Scissors),
            _ => None,
        }
    }
}

fn main() {
    let file = File::open("./res/input_real.txt").unwrap();

    let mut score = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let chars = line.chars().collect::<Vec<char>>();

        let p1 = Choose::from_p1(chars.first().unwrap()).unwrap();
        let mut p2 = Choose::from_p2(chars.last().unwrap()).unwrap();

        match p2 {
            Choose::Rock => 
            match p1 {
                Choose::Rock => p2 = Choose::Scissors,
                Choose::Paper => p2= Choose::Rock,
                Choose::Scissors => p2 = Choose::Paper,
            },
            Choose::Paper => p2 = p1.clone(),
            Choose::Scissors => 
            match p1 {
                Choose::Rock => p2 = Choose::Paper,
                Choose::Paper => p2 = Choose::Scissors,
                Choose::Scissors => p2 = Choose::Rock,
            },
        }

        //println!("{:?}", line);
        //println!("{:?} {:?}", p1, p2);

        match p2 {
            Choose::Rock => score += 1,
            Choose::Paper => score += 2,
            Choose::Scissors => score += 3,
        }

        if p2 == p1 {
            score += 3;
        } else {
            match (p1, p2) {
                (Choose::Rock, Choose::Paper) => score += 6,
                (Choose::Paper, Choose::Scissors) => score += 6,
                (Choose::Scissors, Choose::Rock) => score += 6,
                _ => score += 0,
            }
        }      
    }

    println!("Score: {}", score);
}
