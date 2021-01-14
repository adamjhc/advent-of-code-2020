use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let code: Vec<String> = lines.collect();

    let mut program_counter = 0;
    let mut lines_run: Vec<i32> = Vec::new();
    let mut accumulator = 0;

    while !lines_run.contains(&program_counter) {
        lines_run.push(program_counter);

        // parse line of code
        let line = code.get(program_counter as usize).unwrap();
        let instruction = &line[0..3];
        match instruction {
            "acc" => {
                let sign = line.chars().nth(4).unwrap();
                let amount = &line[5..].parse::<i32>().unwrap();

                if sign == '+' {
                    accumulator += amount
                } else if sign == '-' {
                    accumulator -= amount
                }

                program_counter += 1
            }
            "jmp" => {
                let sign = line.chars().nth(4).unwrap();
                let amount = &line[5..].parse::<i32>().unwrap();

                if sign == '+' {
                    program_counter += amount
                } else if sign == '-' {
                    program_counter -= amount
                }
            }
            _ => program_counter += 1,
        }
    }

    println!("{}", accumulator)
}
