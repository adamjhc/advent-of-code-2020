use std::collections::HashMap;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let numbers_starting = vec![7, 12, 1, 0, 16, 2];

    // HashMap <number spoken, turn when spoken>
    let mut numbers_spoken: HashMap<i32, Vec<i32>> = numbers_starting
        .iter()
        .enumerate()
        .map(|(i, number)| (*number, vec![i as i32]))
        .collect();

    let mut number_previous = numbers_starting[numbers_starting.len() - 1];
    let mut number_spoken = 0;
    for turn in numbers_starting.len() as i32..2020 {
        number_spoken = if numbers_spoken.get(&number_previous).unwrap().len() == 1 {
            0
        } else {
            let past_turns = numbers_spoken.get(&number_previous).unwrap();
            turn - 1 - past_turns[past_turns.len() - 2]
        };

        if numbers_spoken.contains_key(&number_spoken) {
            let turns_spoken = numbers_spoken.get_mut(&number_spoken).unwrap();
            turns_spoken.push(turn);
        } else {
            numbers_spoken.insert(number_spoken, vec![turn]);
        }

        number_previous = number_spoken;
    }

    println!("{}", number_spoken);
}

fn part_2() {
    let numbers_starting = vec![7, 12, 1, 0, 16, 2];

    // HashMap <number spoken, turn when spoken>
    let mut numbers_spoken: HashMap<i32, Vec<i32>> = numbers_starting
        .iter()
        .enumerate()
        .map(|(i, number)| (*number, vec![i as i32]))
        .collect();

    let mut number_previous = numbers_starting[numbers_starting.len() - 1];
    let mut number_spoken = 0;
    for turn in numbers_starting.len() as i32..30_000_000 {
        number_spoken = if numbers_spoken.get(&number_previous).unwrap().len() == 1 {
            0
        } else {
            let past_turns = numbers_spoken.get(&number_previous).unwrap();
            turn - 1 - past_turns[past_turns.len() - 2]
        };

        if numbers_spoken.contains_key(&number_spoken) {
            let turns_spoken = numbers_spoken.get_mut(&number_spoken).unwrap();
            turns_spoken.push(turn);
        } else {
            numbers_spoken.insert(number_spoken, vec![turn]);
        }

        number_previous = number_spoken;
    }

    println!("{}", number_spoken);
}
