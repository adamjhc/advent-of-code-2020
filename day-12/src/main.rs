use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
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

fn part_2() {
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
    let mut waypoint = (10, 1);

    for (action, amount) in instructions {
        match action {
            'N' => waypoint.1 += amount,
            'S' => waypoint.1 -= amount,
            'E' => waypoint.0 += amount,
            'W' => waypoint.0 -= amount,
            'L' | 'R' => {
                let mut rotation = amount;
                if action == 'L' {
                    rotation = 360 - amount;
                }
                waypoint = match rotation {
                    90 => (waypoint.1, -waypoint.0),
                    180 => (-waypoint.0, -waypoint.1),
                    270 => (-waypoint.1, waypoint.0),
                    _ => panic!(),
                }
            }
            'F' => {
                position = (
                    position.0 + amount * waypoint.0,
                    position.1 + amount * waypoint.1,
                )
            }
            _ => panic!(),
        }
    }

    println!("{}", position.0.abs() + position.1.abs())
}
