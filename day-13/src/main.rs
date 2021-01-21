use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
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

fn part_2() {
    let file = File::open("./input.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    // ignore first line
    lines.next();

    let mut bus_no = -1;
    let buses: Vec<(i64, i64)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .into_iter()
        .filter_map(|bus| {
            bus_no += 1;
            if let Ok(bus_id) = bus.parse::<i64>() {
                return Some((bus_no, bus_id));
            }
            None
        })
        .collect();

    // for some reason the sort isn't needed?
    // buses.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut time: i64 = 0;
    let mut inc = buses[0].1;
    for (bus_no, bus_time) in &buses[1..] {
        // (t + i) % bus[i] = 0
        while (time + bus_no) % bus_time != 0 {
            time += inc;
        }

        // adjust for the next modulo
        inc *= bus_time;
    }

    println!("{}", time);
}
