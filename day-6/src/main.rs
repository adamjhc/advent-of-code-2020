use std::fs;

fn main() {
    part_1();
    part_2();
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

fn part_2() {
    let batch = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for passport in batch.split("\n\n").map(str::to_owned) {
        let mut question_intersection: Vec<char> = Vec::new();

        for (i, line) in passport.lines().enumerate() {
            if i == 0 {
                for question in line.chars() {
                    question_intersection.push(question);
                }
            } else {
                question_intersection = question_intersection
                    .into_iter()
                    .filter(|&x| line.find(x) != None)
                    .collect();
            }
        }

        sum += question_intersection.len();
    }

    println!("{}", sum);
}
