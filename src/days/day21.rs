use std::cmp::max;

use cached::{proc_macro::cached};

const INPUT: &str = "Player 1 starting position: 6\nPlayer 2 starting position: 4";

fn process_input() -> (i32, i32) {
    let pos: Vec<i32> = INPUT
        .lines()
        .map(|l| l.split_at(28).1.parse::<i32>().unwrap())
        .collect();
    (pos[0], pos[1])
}

#[cached]
fn quantum_die(p1_score: i32, p1_pos: i32, p2_score: i32, p2_pos: i32) -> (i64, i64) {
    if p1_score >= 21 {
        return (1, 0);
    }
    if p2_score >= 21 {
        return (0, 1);
    }

    let mut result = (0, 0);

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let roll = i + j + k;
                let score = if (p1_pos + roll) % 10 == 0 { 10 } else { (p1_pos + roll) % 10 };

                let (a, b) =  quantum_die(p2_score, p2_pos, p1_score + score, (p1_pos + roll) % 10);
                result.0 += b;
                result.1 += a;
            }
        }
    }

    result
}

pub fn part1() -> usize {
    let (mut p1_pos, mut p1_score) = (process_input().0, 0);
    let (mut p2_pos, mut p2_score) = (process_input().1, 0);

    let mut roll: i32 = 0;
    let mut losing_score = 0;

    for i in (6..).step_by(9) {
        if p1_score >= 1000 || p2_score >= 1000 {
            losing_score = if p1_score >= 1000 { p2_score } else { p1_score };
            break;
        }

        if i % 2 == 0 {
            p1_pos += i % 10;
            p1_score += if p1_pos % 10 == 0 { 10 } else { p1_pos % 10 };
        } else {
            p2_pos += i % 10;
            p2_score += if p2_pos % 10 == 0 { 10 } else { p2_pos % 10 };
        }

        roll += 3;
    }

    (roll * losing_score) as usize
}

pub fn part2() -> usize {
    let (p1_pos, p1_score) = (process_input().0, 0);
    let (p2_pos, p2_score) = (process_input().1, 0);

    let (x, y) = quantum_die(p1_score, p1_pos, p2_score, p2_pos);

    max(x, y) as usize
}
