use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    for i in 25..lines.len() {
        let mut found_pair = false;
        'calc: for j in 0..25 {
            for k in 0..25 {
                if j == k {
                    continue;
                }

                if lines[i - j - 1] + lines[i - k - 1] == lines[i] {
                    found_pair = true;
                    break 'calc;
                }
            }
        }

        if !found_pair {
            println!("{}", lines[i]);
            return;
        }
    }
}
