use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    let years = read_input();

    println!("{}", part_1(&years));
    println!("{}", part_2(&years));
}

fn part_1(years: &Vec<i32>) -> i32 {
    let year_len = years.len();
    for i in 0..year_len {
        for j in 0..year_len {
            if years[i] + years[j] == 2020 {
                return years[i] * years[j];
            }
        }
    }

    panic!()
}

fn part_2(years: &Vec<i32>) -> i32 {
    let year_len = years.len();
    for i in 0..year_len {
        for j in 0..year_len {
            for k in 0..year_len {
                if years[i] + years[j] + years[k] == 2020 {
                    return years[i] * years[j] * years[k];
                }
            }
        }
    }

    panic!()
}

fn read_input() -> Vec<i32> {
    let mut years = Vec::new();
    let lines = read_lines("./input.txt").expect("./input.txt not found");
    for line in lines {
        years.push(line.unwrap().parse::<i32>().unwrap());
    }

    years
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        let years = read_input();

        assert_eq!(part_1(&years), 1006875)
    }

    #[test]
    fn part_2_gives_correct_answer() {
        let years = read_input();

        assert_eq!(part_2(&years), 165026160)
    }
}
