use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    part_1();
    part_2();
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

fn part_2() {
    let input = read_to_string("./input.txt").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();

    // handle rules
    let mut field_rules: HashMap<String, Vec<(_, _)>> = HashMap::new();
    let rules_raw = sections[0].lines();
    let re = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    for rule in rules_raw {
        let captures = re.captures(&rule).unwrap();
        let rule_name = captures.get(1).unwrap().as_str().to_string();
        let rule_first_l = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let rule_first_h = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let rule_secon_l = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let rule_secon_h = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();
        field_rules.insert(
            rule_name,
            vec![(rule_first_l, rule_first_h), (rule_secon_l, rule_secon_h)],
        );
    }

    // gather 'your ticket'
    let your_ticket_raw = sections[1].lines().skip(1).next().unwrap();
    let your_ticket: Vec<i32> = your_ticket_raw
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    // gather 'nearby tickets'
    let mut nearby_tickets: Vec<Vec<i32>> = Vec::new();
    let mut nearby_tickets_raw = sections[2].lines();
    nearby_tickets_raw.next(); // ignore "nearby tickets:" line
    for ticket in nearby_tickets_raw {
        let values: Vec<i32> = ticket
            .split(',')
            .map(|value| value.parse::<i32>().unwrap())
            .collect();

        if values.iter().all(|&value| {
            field_rules
                .iter()
                .any(|(_, rules)| rules.iter().any(|rule| rule.0 <= value && value <= rule.1))
        }) {
            nearby_tickets.push(values);
        }
    }

    // deduct which field corresponds to which values
    let mut field_indices: HashMap<String, Vec<usize>> = HashMap::new();
    //  for each column of values
    //      for each field
    //          for each value
    //              if all values fit the rules then add index to hashmap
    for (field, rules) in field_rules {
        for i in 0..nearby_tickets[0].len() {
            if nearby_tickets.iter().all(|ticket| {
                rules
                    .iter()
                    .any(|rule| rule.0 <= ticket[i] && ticket[i] <= rule.1)
            }) {
                if let Some(indices) = field_indices.get_mut(&field) {
                    indices.push(i)
                } else {
                    field_indices.insert(field.to_string(), vec![i]);
                }
            }
        }
    }

    let mut ordered_fields = field_indices
        .iter()
        .collect::<Vec<(&String, &Vec<usize>)>>();
    ordered_fields.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));

    let mut matched: HashMap<&String, &usize> = HashMap::new();
    let mut found_indices = Vec::new();
    for (field, possibilities) in ordered_fields {
        let index: &usize = possibilities
            .iter()
            .filter(|i| !found_indices.contains(i))
            .collect::<Vec<&usize>>()[0];
        found_indices.push(index);
        matched.insert(field, index);
    }

    // multiply values from my ticket for fields that start with 'departure'
    println!(
        "{:?}",
        matched
            .iter()
            .filter(|(k, _)| k.starts_with("departure"))
            .map(|(_, &v)| your_ticket[*v] as i64)
            .product::<i64>()
    )
}
