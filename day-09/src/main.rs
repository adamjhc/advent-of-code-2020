use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    let invalid_number = part_1(&lines);
    part_2(&lines, invalid_number);
}

fn part_1(lines: &Vec<i64>) -> i64 {
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
            return lines[i];
        }
    }

    0
}

fn part_2(lines: &Vec<i64>, invalid_number: i64) {
    for i in 0..lines.len() {
        let mut sum = lines[i];
        let mut smallest = lines[i];
        let mut largest = lines[i];
        for j in i + 1..lines.len() {
            sum += lines[j];

            if lines[j] < smallest {
                smallest = lines[j]
            }
            if lines[j] > largest {
                largest = lines[j]
            }

            if sum == invalid_number {
                println!("{}", smallest + largest);
                return;
            }

            if sum > invalid_number {
                break;
            }
        }
    }
}
