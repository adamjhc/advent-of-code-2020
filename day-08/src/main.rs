use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
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

fn part_2() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());
    let code: Vec<String> = lines.collect();

    let mut has_terminated = false;
    let mut accumulator = 0;
    let mut skip = 0;

    while has_terminated == false {
        let mut program_counter = 0;
        let mut lines_run: Vec<i32> = Vec::new();
        accumulator = 0;
        let mut skip_current = skip;

        while !lines_run.contains(&program_counter) {
            if program_counter as usize == code.len() {
                has_terminated = true;
                break;
            }

            lines_run.push(program_counter);

            // parse line of code
            let line = code.get(program_counter as usize).unwrap();
            let mut instruction = &line[0..3];
            instruction = match (skip_current, instruction) {
                (0, "jmp") => {
                    skip_current -= 1;
                    "nop"
                }
                (0, "nop") => {
                    skip_current -= 1;
                    "jmp"
                }
                (_, "jmp") | (_, "nop") => {
                    skip_current -= 1;
                    instruction
                }
                (_, _) => instruction,
            };

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

        skip += 1
    }

    println!("{}", accumulator)
}
