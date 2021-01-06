use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let batch = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for passport in batch.split("\n\n").map(str::to_owned) {
        let mut unique_questions = String::new();

        for line in passport.lines() {
            for question in line.chars() {
                if !unique_questions.contains(question) {
                    unique_questions.push(question);
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
