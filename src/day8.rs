use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day8.txt");

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|line| {
            line.split_once(" | ")
                .unwrap()
                .1
                .split_whitespace()
                .map(|f| f.len())
                .filter(|f| *f < 5 || *f == 7)
                .count()
        })
        .sum::<usize>()
}

#[rustfmt::skip]
pub fn part2() -> usize {
    let mut patterns = HashMap::<char, &str>::new();
    let mut num_sum: usize = 0;

    for line in INPUT.lines() {
        let mut sum = String::new();
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
                    } else if patterns.get(&'4').unwrap().chars().filter(|c| output.contains(*c)).count() == 3 {
                        sum.push('5');
                    } else {
                        sum.push('2');
                    }
                },
                6 => {
                    if patterns.get(&'4').unwrap().chars().all(|c| output.contains(c)) {
                        sum.push('9');
                    } else if patterns.get(&'1').unwrap().chars().all(|c| output.contains(c)) {
                        sum.push('0');
                    } else {
                        sum.push('6');
                    }
                },
                7 => sum.push('8'),
                _ => (),
            }
        }
        println!("{}", &sum);
        num_sum += usize::from_str_radix(sum.as_str(), 10).unwrap();
    }

    num_sum
}
