use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part 1: {}", count_trees_in_path(3, 1));
    println!(
        "Part 2: {}",
        count_trees_in_path(1, 1)
            * count_trees_in_path(3, 1)
            * count_trees_in_path(5, 1)
            * count_trees_in_path(7, 1)
            * count_trees_in_path(1, 2)
    )
}

fn count_trees_in_path(right: usize, down: usize) -> i64 {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut map: Vec<Vec<char>> = Vec::new();
    for (i, line) in lines.enumerate() {
        map.push(Vec::new());
        for chr in line.unwrap().chars() {
            map[i].push(chr)
        }
    }

    let mut trees = 0;

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

    return trees;
}
