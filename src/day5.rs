use std::cmp::max;

const INPUT: &str = include_str!("inputs/day5.txt");

fn get_intersections(diagonal: bool) -> usize {
    let mut field: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for l in INPUT.lines() {
        let (p1, p2) = l.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x2, y2) = p2.split_once(",").unwrap();

        let m1: i32 = x1.parse().unwrap();
        let m2: i32 = x2.parse().unwrap();

        let n1: i32 = y1.parse().unwrap();
        let n2: i32 = y2.parse().unwrap();

        if !diagonal {
            if !(x1 == x2 || y1 == y2) {
                continue;
            }
        }

        let dx: i32 = m1 - m2;
        let dy: i32 = n1 - n2;

        let signum_x = (m2 - m1).signum();
        let signum_y = (n2 - n1).signum();

        for i in 0..=max(dx.abs(), dy.abs()) {
            field[(n1 + i * signum_y) as usize][(m1 + i * signum_x) as usize] += 1;
        }
    }

    field.iter().map(|r| r.iter().filter(|n| **n >= 2).count()).sum()
}

pub fn part1() -> usize {
    get_intersections(false)
}

pub fn part2() -> usize {
    get_intersections(true)
}