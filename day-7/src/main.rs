use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    // build up graph of bags
    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    let rule_regex = Regex::new(r"(\w+ \w+) bags contain (.*)\.").unwrap();

    for line in lines {
        let rule_captures = rule_regex.captures(&line).unwrap();
        let bag_outside = rule_captures.get(1).unwrap().as_str();

        let contains_regex = Regex::new(r"(?:\d (\w+ \w+))").unwrap();
        let contains = rule_captures.get(2).unwrap().as_str();
        let contains_captures = contains_regex.captures_iter(&contains);
        for cap in contains_captures {
            let bag_inside = cap.get(1).unwrap().as_str();

            if bags.contains_key(bag_inside) {
                bags.get_mut(bag_inside)
                    .unwrap()
                    .push(bag_outside.to_string())
            } else {
                bags.insert(bag_inside.to_string(), vec![bag_outside.to_string()]);
            }
        }
    }

    println!("{}", count_bag_colours(&bags, "shiny gold").len());
}

fn count_bag_colours(bags: &HashMap<String, Vec<String>>, bag_inside: &str) -> HashSet<String> {
    match bags.get(bag_inside) {
        Some(bags_outside) => {
            let mut bag_colours: HashSet<String> = bags_outside.iter().cloned().collect();
            for bag in bags_outside {
                bag_colours.extend(count_bag_colours(bags, bag));
            }
            bag_colours
        }
        None => HashSet::new(),
    }
}

fn part_2() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    // build up graph of bags
    let mut bags: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    let rule_regex = Regex::new(r"(\w+ \w+) bags contain (.*)\.").unwrap();

    for line in lines {
        let rule_captures = rule_regex.captures(&line).unwrap();
        let bag_outside = rule_captures.get(1).unwrap().as_str();

        let contains_regex = Regex::new(r"(?:(\d) (\w+ \w+))").unwrap();
        let contains = rule_captures.get(2).unwrap().as_str();
        let contains_captures = contains_regex.captures_iter(&contains);
        for cap in contains_captures {
            let bag_inside_count = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let bag_inside = cap.get(2).unwrap().as_str();

            if bags.contains_key(bag_outside) {
                bags.get_mut(bag_outside)
                    .unwrap()
                    .push((bag_inside_count, bag_inside.to_string()))
            } else {
                bags.insert(
                    bag_outside.to_string(),
                    vec![(bag_inside_count, bag_inside.to_string())],
                );
            }
        }
    }

    println!("{}", count_bags_inside(&bags, "shiny gold"));
}

fn count_bags_inside(bags: &HashMap<String, Vec<(i32, String)>>, bag_outside: &str) -> i32 {
    match bags.get(bag_outside) {
        Some(bags_inside) => bags_inside.iter().fold(0, |count, bag| {
            count + bag.0 + bag.0 * count_bags_inside(bags, &bag.1)
        }),
        None => 0,
    }
}
