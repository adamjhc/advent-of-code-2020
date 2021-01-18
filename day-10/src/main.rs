use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let mut lines: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    lines.sort();

    let mut jolt_1 = 0;
    let mut jolt_3 = 0;

    // account for starting joltage of 0
    if lines[0] == 1 {
        jolt_1 += 1
    } else {
        jolt_3 += 1
    }

    for i in 1..lines.len() {
        if lines[i] - lines[i - 1] == 1 {
            jolt_1 += 1
        } else {
            jolt_3 += 1
        }
    }

    // account for device adapter which is always 3 above
    jolt_3 += 1;

    println!("{}", jolt_1 * jolt_3)
}

fn part_2() {
    let file = File::open("./input.txt").unwrap();
    let mut lines: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    lines.push(0);
    lines.sort();
    lines.push(lines[lines.len() - 1] + 3);

    let mut results: HashMap<i64, i64> = HashMap::new();

    let arrangements = count_arrangements(0, &lines, &mut results);

    println!("{}", arrangements)
}

fn count_arrangements(joltage: i64, joltages: &Vec<i64>, results: &mut HashMap<i64, i64>) -> i64 {
    // if result has been memoised return it
    if results.contains_key(&joltage) {
        return *results.get(&joltage).unwrap();
    }

    // else calculate actual result
    if joltage == joltages[joltages.len() - 1] {
        return 1;
    }
    if !joltages.contains(&joltage) {
        return 0;
    }

    let arrangements = count_arrangements(joltage + 1, joltages, results)
        + count_arrangements(joltage + 2, joltages, results)
        + count_arrangements(joltage + 3, joltages, results);

    // save result to dictionary
    results.insert(joltage, arrangements);

    arrangements
}
