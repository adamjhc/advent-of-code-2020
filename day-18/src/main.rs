use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let expressions: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut sum = 0;
    for mut expression in expressions {
        expression = expression.replace("(", "( ");
        expression = expression.replace(")", " )");
        let elements: Vec<&str> = expression.split_whitespace().collect();

        sum += evaluate(&elements, 0).0;
    }

    println!("{}", sum);
}

enum Op {
    Add,
    Mult,
}

fn evaluate(elements: &Vec<&str>, start_index: usize) -> (i64, usize) {
    let mut total = 0;
    let mut next_op = Op::Add;
    let mut new_index = 0;

    for i in start_index..elements.len() {
        if i < new_index {
            continue;
        }

        match elements[i] {
            "(" => {
                let result = evaluate(elements, i + 1);
                let value = result.0;
                new_index = result.1;
                match next_op {
                    Op::Mult => total *= value,
                    Op::Add => total += value,
                }
            }
            ")" => return (total, i + 1),
            "+" => next_op = Op::Add,
            "*" => next_op = Op::Mult,
            val => {
                let value = val.parse::<i64>().unwrap();
                match next_op {
                    Op::Mult => total *= value,
                    Op::Add => total += value,
                }
            }
        }
    }

    (total, 0)
}
