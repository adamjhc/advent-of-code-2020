use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let earliest_time = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let buses: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .into_iter()
        .filter(|s| s != &"x")
        .map(|t| t.parse::<i32>().unwrap())
        .collect();

    let mut current_time = earliest_time;
    loop {
        for bus_time in &buses {
            if current_time % bus_time == 0 {
                println!("{}", bus_time * (current_time - earliest_time));
                return;
            }
        }

        current_time += 1;
    }
}
