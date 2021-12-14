use std::{collections::HashMap, iter::FromIterator};

const INPUT: &str = include_str!("../inputs/day14.txt");

fn polymer_n(steps: usize) -> usize {
    let starting_point: &str = INPUT.lines().nth(0).unwrap();

    let mut insertion_rules: HashMap<&str, &str> = HashMap::new();
    let mut occurences: HashMap<String, usize> = HashMap::new();
    let mut letter_occurences: HashMap<String, usize> = HashMap::new();

    for rule in INPUT.lines().skip(2) {
        let (key, value) = rule.split_once(" -> ").unwrap();
        insertion_rules.insert(key, value);
    }

    for pair in starting_point.chars().collect::<Vec<_>>().windows(2) {
        occurences.insert(String::from_iter(pair), 1);
    }

    starting_point.chars().for_each(|c| *letter_occurences.entry(String::from(c)).or_insert(0) += 1);

    for _ in 0..steps {
        let mut add_list: HashMap<String, usize> = HashMap::new();
        let mut rm_list: HashMap<String, usize> = HashMap::new();

        for pair in occurences.keys() {
            let new_pair = pair.chars().nth(0).unwrap().to_string() + insertion_rules[pair.as_str()]; 
            let new_pair2 = insertion_rules[pair.as_str()].to_string() + &pair.chars().nth(1).unwrap().to_string();

            *add_list.entry(new_pair).or_insert(0) += occurences[pair]; 
            *add_list.entry(new_pair2).or_insert(0) += occurences[pair]; 
                
            *rm_list.entry(pair.to_string()).or_insert(0) += occurences[pair]; 

            *letter_occurences.entry(insertion_rules[pair.as_str()].to_string()).or_insert(0) += occurences[pair];
        } 

        for (x, y) in &add_list {
            *occurences.entry(x.to_string()).or_insert(0) += y;
        }
        for (x, y) in &rm_list {
            *occurences.entry(x.to_string()).or_insert(0) -= y;
        }
    }
    
    let max = letter_occurences.values().max().unwrap();
    let min = letter_occurences.values().min().unwrap();
    max - min
}

pub fn part1() -> usize {
    polymer_n(10)
}

pub fn part2() -> usize {
    polymer_n(40)
}