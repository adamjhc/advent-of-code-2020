use regex::Regex;
use std::fs::read_to_string;

fn main() {
    part_1();
}

fn part_1() {
    let input = read_to_string("./input.txt").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();

    // handle rules
    let mut rules = Vec::new();
    let rules_raw = sections[0].lines();
    let re = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    for rule in rules_raw {
        let captures = re.captures(&rule).unwrap();
        let rule_first_l = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let rule_first_h = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let rule_secon_l = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let rule_secon_h = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();
        rules.push((rule_first_l, rule_first_h));
        rules.push((rule_secon_l, rule_secon_h));
    }

    // ignore 'your ticket'
    let _ = sections[1];

    // gather 'nearby tickets' and count invalids
    let mut invalid_sum = 0;
    let mut nearby_tickets = sections[2].lines();
    nearby_tickets.next(); // ignore "nearby tickets:" line
    for ticket in nearby_tickets {
        let values: Vec<i32> = ticket
            .split(',')
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        for value in values {
            if !rules.iter().any(|rule| rule.0 <= value && value <= rule.1) {
                invalid_sum += value;
            }
        }
    }

    println!("{}", invalid_sum);
}
