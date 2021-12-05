use std::cmp::{max, min};

const INPUT: &str = include_str!("inputs/day5.txt");

fn get_intersections(diagonal: bool) -> usize {
    let mut field: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for l in INPUT.lines() {
        let (p1, p2) = l.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x2, y2) = p2.split_once(",").unwrap();

        let m1: usize = x1.parse().unwrap();
        let m2: usize = x2.parse().unwrap();

        let n1: usize = y1.parse().unwrap();
        let n2: usize = y2.parse().unwrap();

        if x1 == x2 || y1 == y2 {
            if m1 != m2 {
                for i in min(m1, m2)..=max(m1, m2) {
                    field[n1][i] += 1;
                }
            }
            if n1 != n2 {
                for i in min(n1, n2)..=max(n1, n2) {
                    field[i][m1] += 1;
                }
            }
        } else {
            if diagonal {
                let difference = max(m1, m2) - min(m1, m2);

                let x = min(m1, m2);
                let y = if m1 < m2 { n1 } else { n2 };

                if n1 < n2 {
                    for i in 0..=difference {
                        field[if y == n2 { y - i } else { y + i }][x + i] += 1;
                    }
                }
                if n2 < n1 {
                    for i in 0..=difference {
                        field[if y == n2 { y + i } else { y - i }][x + i] += 1;
                    }
                }
            }
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
