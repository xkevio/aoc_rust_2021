use std::cmp::max;

const INPUT: &str = include_str!("../inputs/day5.txt");

fn get_intersections(diagonal: bool) -> usize {
    let mut field: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for l in INPUT.lines() {
        let pairs: Vec<i32> = l
            .split(|c: char| !c.is_numeric())
            .filter_map(|c| c.parse::<i32>().ok())
            .collect();
        let (x1, y1, x2, y2) = (pairs[0], pairs[1], pairs[2], pairs[3]);

        if !(diagonal || x1 == x2 || y1 == y2) {
            continue;
        }

        let dx: i32 = x1 - x2;
        let dy: i32 = y1 - y2;

        let signum_x = (x2 - x1).signum();
        let signum_y = (y2 - y1).signum();

        for i in 0..=max(dx.abs(), dy.abs()) {
            field[(y1 + i * signum_y) as usize][(x1 + i * signum_x) as usize] += 1;
        }
    }

    field
        .iter()
        .map(|r| r.iter().filter(|n| **n >= 2).count())
        .sum()
}

pub fn part1() -> usize {
    get_intersections(false)
}

pub fn part2() -> usize {
    get_intersections(true)
}
