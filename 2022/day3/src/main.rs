use std::collections::HashSet;
use std::fs::*;
use std::io::*;

fn share_char(a: &str, b: &str) -> Option<char> {
    let set: HashSet<char> = a.chars().collect();

    let mut ch: Option<char> = None;

    b.chars().find(|c| {
        if set.contains(c) {
            ch = Some(*c);
            true
        } else {
            false
        }
    });

    ch
}

fn share_char_3(a: &str, b: &str, c: &str) -> Option<char> {
    let seta: HashSet<char> = a.chars().collect();
    let setb: HashSet<char> = b.chars().collect();

    let mut ch: Option<char> = None;

    c.chars().find(|c| {
        if seta.contains(c) && setb.contains(c) {
            ch = Some(*c);
            true
        } else {
            false
        }
    });

    ch
}

fn decode(ch: char) -> Option<u8> {
    let i = ch as u8;

    match ch {
        'a'..='z' => Some(i - b'a' + 1),
        'A'..='Z' => Some(i - b'A' + 27),
        _ => None,
    }
}

fn main() {
    let file = File::open("./res/input_real.txt").unwrap();

    let mut list : Vec<String> = Vec::new();

    let mut sum = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        list.push(line);

        if list.len() >= 3 {
            
            sum += decode(share_char_3(&list[0], &list[1], &list[2]).unwrap()).unwrap() as u32;
            

            list.clear();
        }
    }

    // for line in BufReader::new(file).lines() {
    //     let line = line.unwrap();

    //     let first = line.get(..line.len() / 2).unwrap();
    //     let second = line.get(line.len() / 2..).unwrap();

    //     sum += decode(share_char(first, second).unwrap()).unwrap() as u32;
    // }

    println!("{}", sum);
}
