use regex::Regex;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let batch = fs::read_to_string("./input.txt").unwrap();
    let mut valid = 0;
    for passport in batch.split("\n\n").map(str::to_owned) {
        let mut attributes = 0;
        for pairs in passport.split_whitespace().map(str::to_owned) {
            let pair = pairs.split(':').map(str::to_owned).collect::<Vec<_>>();
            match pair[0].as_ref() {
                "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => attributes += 1,
                "cid" => (),
                _ => (),
            }
        }

        if attributes == 7 {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn part_2() {
    let batch = fs::read_to_string("./input.txt").unwrap();
    let mut valid = 0;
    for passport in batch.split("\n\n").map(str::to_owned) {
        let mut attributes = 0;
        for pairs in passport.split_whitespace().map(str::to_owned) {
            let pair = pairs.split(':').map(str::to_owned).collect::<Vec<_>>();
            match pair[0].as_ref() {
                "byr" => {
                    let birth_year = pair[1].parse::<i32>().unwrap();
                    if pair[1].len() == 4 && birth_year >= 1920 && birth_year <= 2002 {
                        attributes += 1;
                    }
                }
                "iyr" => {
                    let issue_year = pair[1].parse::<i32>().unwrap();
                    if pair[1].len() == 4 && issue_year >= 2010 && issue_year <= 2020 {
                        attributes += 1;
                    }
                }
                "eyr" => {
                    let expiration_year = pair[1].parse::<i32>().unwrap();
                    if pair[1].len() == 4 && expiration_year >= 2020 && expiration_year <= 2030 {
                        attributes += 1;
                    }
                }
                "hgt" => {
                    let re = Regex::new(r"^(\d*)(\w*)$").unwrap();
                    let captures = re.captures(&pair[1]).unwrap();
                    let height = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let format = captures.get(2).unwrap().as_str();
                    if (format == "cm" && height >= 150 && height <= 193)
                        || (format == "in" && height >= 59 && height <= 76)
                    {
                        attributes += 1
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                    if re.is_match(&pair[1]) {
                        attributes += 1;
                    }
                }
                "ecl" => match pair[1].as_ref() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => attributes += 1,
                    _ => (),
                },
                "pid" => {
                    let re = Regex::new(r"^[0-9]{9}$").unwrap();
                    if re.is_match(&pair[1]) {
                        attributes += 1
                    }
                }
                _ => (),
            }
        }

        if attributes == 7 {
            valid += 1;
        }
    }

    println!("{}", valid);
}
