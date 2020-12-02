use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut years = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(year) = line {
                years.push(year.parse::<i32>().unwrap());
            }
        }

        let year_len = years.len();
        for i in 0..year_len {
            for j in 0..year_len {
                if years[i] + years[j] == 2020 {
                    println!("{}", years[i] * years[j]);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}