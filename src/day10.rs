use std::collections::HashMap;

const INPUT: &str = include_str!("inputs/day10.txt");

fn get_pairs() -> HashMap<char, char> {
    HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')])
}

fn is_corrupted(line: &str, pairs: &HashMap<char, char>) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();
    for ch in line.chars() {
        if pairs.contains_key(&ch) {
            stack.push(ch);
        } else {
            if ch == pairs[stack.last().unwrap()] {
                stack.pop();
            } else {
                return Some(ch);
            }
        }
    }
    None
}

pub fn part1() -> usize {
    let mut sum: usize = 0;

    for line in INPUT.lines() {
        if let Some(ch) = is_corrupted(line, &get_pairs()) {
            match ch {
                ')' => sum += 3,
                ']' => sum += 57,
                '}' => sum += 1197,
                '>' => sum += 25137,
                _ => (),
            }
        }
    }

    sum
}

pub fn part2() -> usize {
    let pairs = get_pairs();
    let mut scores: Vec<usize> = Vec::new();

    for line in INPUT.lines().filter(|l| is_corrupted(*l, &pairs).is_none()) {
        let mut stack: Vec<char> = Vec::new();
        for ch in line.chars() {
            if pairs.contains_key(&ch) {
                stack.push(pairs[&ch]);
            } else {
                if ch == *stack.last().unwrap() {
                    stack.pop();
                }
            }
        }

        let mut score: usize = 0;
        for c in stack.iter().rev() {
            score *= 5;
            match c {
                ')' => score += 1,
                ']' => score += 2,
                '}' => score += 3,
                '>' => score += 4,
                _ => (),
            }
        }

        scores.push(score);
    }

    scores.sort();
    scores[scores.len() / 2]
}
