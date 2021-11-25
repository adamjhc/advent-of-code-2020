use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Rule {
    Single(char),
    Expand(ExpandedRule),
}

type SubRule = Vec<usize>;

#[derive(Debug)]
enum ExpandedRule {
    Single(SubRule),
    Double((SubRule, SubRule)),
}

fn main() {
    let (mut rules, messages) = read_rules_and_messages();

    println!(
        "Part 1: {}",
        count_messages_matching_zero(&rules, &messages)
    );

    rules.insert(
        8,
        Rule::Expand(ExpandedRule::Double((vec![42], vec![42, 8]))),
    );
    rules.insert(
        11,
        Rule::Expand(ExpandedRule::Double((vec![42, 31], vec![42, 11, 31]))),
    );

    println!(
        "Part 2: {}",
        count_messages_matching_zero(&rules, &messages),
    );
}

fn count_messages_matching_zero(rules: &HashMap<usize, Rule>, messages: &Vec<String>) -> usize {
    // generate messages that match rule 0
    //      recursively follow tree of rules starting at 0
    let rule_zero = rules.get(&0).unwrap();
    let rule_zero_messages = parse(&rules, rule_zero);

    // count number of messages that match those generated
    let rule_zero_matches = messages.iter().fold(0, |count, message| {
        if rule_zero_messages.contains(message) {
            count + 1
        } else {
            count
        }
    });

    rule_zero_matches
}

fn read_rules_and_messages() -> (HashMap<usize, Rule>, Vec<String>) {
    let file = File::open("./input.txt").unwrap();
    let mut lines = BufReader::new(file).lines().into_iter();

    let mut rules: HashMap<usize, Rule> = HashMap::new();

    // read in all rules
    let regex_set = RegexSet::new(&[
        r"^(\d+): (\d+)$",
        r"^(\d+): (\d+) (\d+)$",
        r"^(\d+): (\d+) \| (\d+)$",
        r"^(\d+): (\d+) (\d+) \| (\d+) (\d+)$",
        r#"^(\d+): "([a-z])""#,
    ])
    .unwrap();

    for line in &mut lines {
        if let Ok(rule) = line {
            if rule.is_empty() {
                break;
            }

            let matches = regex_set.matches(&rule);
            if matches.matched(0) {
                let regex = Regex::new(&regex_set.patterns()[0]).unwrap();
                let captures = regex.captures(&rule).unwrap();

                let rule_number: usize = captures.get(1).unwrap().as_str().parse().unwrap();

                let sub_rule: usize = captures.get(2).unwrap().as_str().parse().unwrap();

                rules.insert(
                    rule_number,
                    Rule::Expand(ExpandedRule::Single(vec![sub_rule])),
                );
            } else if matches.matched(1) {
                let regex = Regex::new(&regex_set.patterns()[1]).unwrap();
                let captures = regex.captures(&rule).unwrap();

                let rule_number: usize = captures.get(1).unwrap().as_str().parse().unwrap();

                let sub_rule1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                let sub_rule2: usize = captures.get(3).unwrap().as_str().parse().unwrap();

                rules.insert(
                    rule_number,
                    Rule::Expand(ExpandedRule::Single(vec![sub_rule1, sub_rule2])),
                );
            } else if matches.matched(2) {
                let regex = Regex::new(&regex_set.patterns()[2]).unwrap();
                let captures = regex.captures(&rule).unwrap();

                let rule_number: usize = captures.get(1).unwrap().as_str().parse().unwrap();

                let sub_rule1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
                let sub_rule2: usize = captures.get(3).unwrap().as_str().parse().unwrap();

                rules.insert(
                    rule_number,
                    Rule::Expand(ExpandedRule::Double((vec![sub_rule1], vec![sub_rule2]))),
                );
            } else if matches.matched(3) {
                let regex = Regex::new(&regex_set.patterns()[3]).unwrap();
                let captures = regex.captures(&rule).unwrap();

                let rule_number: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let mut captures_iter = captures.iter();
                captures_iter.next();
                captures_iter.next();

                let sub_rules: SubRule = captures_iter
                    .map(|a| a.unwrap().as_str().trim().parse::<usize>().unwrap())
                    .collect();

                rules.insert(
                    rule_number,
                    Rule::Expand(ExpandedRule::Double((
                        (sub_rules[0..2]).to_vec(),
                        (sub_rules[2..4]).to_vec(),
                    ))),
                );
            } else if matches.matched(4) {
                let regex = Regex::new(&regex_set.patterns()[4]).unwrap();
                let captures = regex.captures(&rule).unwrap();

                let rule_number: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                let character: char = captures.get(2).unwrap().as_str().parse().unwrap();

                rules.insert(rule_number, Rule::Single(character));
            }
        }
    }

    // read in all messages
    let messages: Vec<String> = lines.map(Result::unwrap).collect();

    (rules, messages)
}

fn parse(rules: &HashMap<usize, Rule>, rule: &Rule) -> Vec<String> {
    match rule {
        Rule::Single(character) => vec![character.to_string()],
        Rule::Expand(ExpandedRule::Single(subrule)) => collect_messages(rules, subrule),
        Rule::Expand(ExpandedRule::Double(subrules)) => {
            let mut messages = collect_messages(rules, &subrules.0);
            messages.append(&mut collect_messages(rules, &subrules.1));
            messages
        }
    }
}

fn collect_messages(rules: &HashMap<usize, Rule>, subrule: &Vec<usize>) -> Vec<String> {
    let mut messages: Vec<String> = Vec::new();
    for rule_num in subrule {
        let mut messages_new: Vec<String> = Vec::new();
        let submessages = parse(rules, rules.get(rule_num).unwrap());

        if messages.is_empty() {
            messages = submessages;
        } else {
            for message in &messages {
                for submessage in &submessages {
                    messages_new.push(message.to_owned() + submessage);
                }
            }
            messages = messages_new;
        }
    }

    messages
}

#[cfg(test)]
mod day_19_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        let (rules, messages) = read_rules_and_messages();

        assert_eq!(count_messages_matching_zero(&rules, &messages), 272);
    }
}
