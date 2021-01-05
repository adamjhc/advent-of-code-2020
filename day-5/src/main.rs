use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut highest_seat_id = 0;
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();

        // calculate row
        let mut lower = 0;
        let mut upper = 127;
        for _ in 0..6 {
            let row_char = chars.next().unwrap();
            let middle = (lower + upper) / 2;
            if row_char == 'F' {
                upper = middle;
            } else if row_char == 'B' {
                lower = middle + 1;
            }
        }

        let row = if chars.next().unwrap() == 'F' {
            lower
        } else {
            upper
        };

        // calculate column
        let mut lower = 0;
        let mut upper = 7;
        for _ in 8..10 {
            let column_char = chars.next().unwrap();
            let middle = (lower + upper) / 2;
            if column_char == 'L' {
                upper = middle;
            } else if column_char == 'R' {
                lower = middle + 1;
            }
        }

        let column = if chars.next().unwrap() == 'L' {
            lower
        } else {
            upper
        };

        let seat_id = row * 8 + column;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id
        }
    }

    println!("{}", highest_seat_id)
}

fn part_2() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut seats: Vec<i32> = vec![0; 1024];
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();

        // calculate row
        let mut lower = 0;
        let mut upper = 127;
        for _ in 0..6 {
            let row_char = chars.next().unwrap();
            let middle = (lower + upper) / 2;
            if row_char == 'F' {
                upper = middle;
            } else if row_char == 'B' {
                lower = middle + 1;
            }
        }

        let row = if chars.next().unwrap() == 'F' {
            lower
        } else {
            upper
        };

        // calculate column
        let mut lower = 0;
        let mut upper = 7;
        for _ in 8..10 {
            let column_char = chars.next().unwrap();
            let middle = (lower + upper) / 2;
            if column_char == 'L' {
                upper = middle;
            } else if column_char == 'R' {
                lower = middle + 1;
            }
        }

        let column = if chars.next().unwrap() == 'L' {
            lower
        } else {
            upper
        };

        let seat_id = row * 8 + column;
        seats[seat_id] = 1;
    }

    for (i, is_filled) in seats.iter().enumerate() {
        if *is_filled == 0 {
            println!("{}", i)
        }
    }
}
