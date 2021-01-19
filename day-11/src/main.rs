use std::fs::File;

use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let mut seats: Vec<Vec<Seat>> = BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    '.' => Seat::Floor,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    loop {
        let mut seats_changed = false;
        let mut seats_new = seats.clone();

        // Run rules on each seat
        for row in 0..seats.len() {
            for col in 0..seats[row].len() {
                let mut seats_occupied = 0;
                for i in 0..3 as usize {
                    for j in 0..3 as usize {
                        if (i == 1 && j == 1)
                            || (row as isize + i as isize - 1 < 0)
                            || (col as isize + j as isize - 1 < 0)
                            || (row + i - 1 > seats.len())
                            || (col + j - 1 > seats[row].len())
                        {
                            continue;
                        }

                        if let Some(seats_row) = seats.get(row + i - 1) {
                            if let Some(Seat::Occupied) = seats_row.get(col + j - 1) {
                                seats_occupied += 1;
                            }
                        }
                    }
                }

                match seats[row][col] {
                    Seat::Empty if seats_occupied == 0 => {
                        seats_new[row][col] = Seat::Occupied;
                        seats_changed = true;
                    }
                    Seat::Occupied if seats_occupied >= 4 => {
                        seats_new[row][col] = Seat::Empty;
                        seats_changed = true;
                    }
                    _ => {}
                }
            }
        }

        seats = seats_new;

        if !seats_changed {
            break;
        }
    }

    // count occupied chairs
    println!(
        "{}",
        seats.iter().fold(0, |count_row, row| count_row
            + row.iter().fold(0, |count_col, seat| {
                if seat == &Seat::Occupied {
                    count_col + 1
                } else {
                    count_col
                }
            }))
    )
}
