use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut map: Vec<Vec<char>> = Vec::new();
    for (i, line) in lines.enumerate() {
        map.push(Vec::new());
        let l = line.unwrap();
        for chr in l.chars() {
            map[i].push(chr)
        }
    }

    let mut trees = 0;

    let right = 3;
    let down = 1;

    let mut row = 0;
    let mut col = 0;

    let col_size = map[0].len();
    while row < map.len() {
        if map[row][col] == '#' {
            trees += 1;
        }

        row += down;

        col += right;
        col %= col_size;
    }

    println!("{}", trees)
}
