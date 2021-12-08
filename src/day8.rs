use std::{collections::HashMap};

const INPUT: &str = include_str!("inputs/day8.txt");

pub fn part1() -> usize {
    let mut counter: usize = 0;

    for line in INPUT.lines() {
        let (_, outputs) = line.split_once(" | ").unwrap();
        for output in outputs.split_whitespace() {
            if output.len() == 2 || output.len() == 4 || output.len() == 3 || output.len() == 7 {
                counter += 1;
            }
        }
    }

    counter
}

pub fn part2() -> usize {
    let mut sum = String::new();
    let mut patterns = HashMap::<char, &str>::new();

    for line in INPUT.lines() {
        let (inputs, outputs) = line.split_once(" | ").unwrap();
        for input in inputs.split_whitespace() {
            match input.len() {
                2 => {
                    patterns.insert('1', input);
                }
                3 => {
                    patterns.insert('7', input);
                }
                4 => {
                    patterns.insert('4', input);
                }
                7 => {
                    patterns.insert('8', input);
                }
                _ => {},
            }
        }

        for output in outputs.split_whitespace() {
            match output.len() {
                2 => sum.push('1'),
                3 => sum.push('7'),
                4 => sum.push('4'),
                5 => {
                    if patterns.get(&'1').unwrap().chars().all(|c| output.contains(c)) {
                        sum.push('3');
                    } else if patterns.get(&'1').unwrap().chars().any(|c| output.contains(c)) {
                        sum.push('2');
                    } else {
                        sum.push('5');
                    }
                },
                _ => (),
            }
        }
    }

    0
    //usize::from_str_radix(sum.as_str(), 10).unwrap()
}