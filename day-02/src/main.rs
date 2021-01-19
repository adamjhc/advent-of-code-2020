use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let re = Regex::new(r"^(\d{1,2})-(\d{1,2}) (.): (.*)$").unwrap();

    let mut valid = 0;
    for line in lines {
        let l = line.unwrap();
        let caps = re.captures(&l).unwrap();

        let min = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let max = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let cha = caps.get(3).unwrap().as_str().parse::<char>().unwrap();
        let pas = caps.get(4).unwrap().as_str();

        let mut count = 0;
        for ch in pas.chars() {
            if ch == cha {
                count += 1;
            }
        }

        if count >= min && count <= max {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn part_2() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let re = Regex::new(r"^(\d{1,2})-(\d{1,2}) (.): (.*)$").unwrap();

    let mut valid = 0;
    for line in lines {
        let l = line.unwrap();
        let caps = re.captures(&l).unwrap();

        let pos1 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let pos2 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let cha = caps.get(3).unwrap().as_str().parse::<char>().unwrap();
        let pas = caps.get(4).unwrap().as_str();

        if (pas.chars().nth(pos1 - 1).unwrap() == cha) ^ (pas.chars().nth(pos2 - 1).unwrap() == cha) {
            valid += 1;
        }
    }

    println!("{}", valid);
}
