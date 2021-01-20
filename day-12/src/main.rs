use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let instructions: Vec<(char, i32)> = BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            (
                line.chars().nth(0).unwrap(),
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut position = (0, 0);
    let mut facing = 90;

    for (action, amount) in instructions {
        match action {
            'N' => position.0 += amount,
            'S' => position.0 -= amount,
            'E' => position.1 += amount,
            'W' => position.1 -= amount,
            'L' => facing -= amount,
            'R' => facing += amount,
            'F' => {
                if facing >= 360 {
                    facing -= 360
                }
                if facing < 0 {
                    facing += 360
                }

                match facing {
                    0 => position.0 += amount,
                    90 => position.1 += amount,
                    180 => position.0 -= amount,
                    270 => position.1 -= amount,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    println!("{}", position.0.abs() + position.1.abs())
}
