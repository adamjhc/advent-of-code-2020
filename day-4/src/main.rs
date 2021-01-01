use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let batch = fs::read_to_string("./input.txt").unwrap();
    let batch2 = batch.to_owned();
    let mut valid = 0;
    for passport in batch2.split("\n\n").map(str::to_owned) {
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
