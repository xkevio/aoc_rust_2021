use min_max_heap::MinMaxHeap;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day15.txt");

type Node = (u32, usize, usize);

fn parse_input() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .collect()
}

fn path(start: &Node, goal: &Node, array: &[Vec<u32>]) -> u32 {
    let mut dist: HashMap<(usize, usize), u32> = HashMap::new();
    let mut heap = MinMaxHeap::new();

    for i in 0..array.len() {
        for j in 0..array[0].len() {
            dist.insert((i, j), u32::MAX);
        }
    }

    heap.push(*start);

    while let Some((cost, i, j)) = heap.pop_min() {
        if goal.1 == i && goal.2 == j {
            return cost;
        }

        if cost > dist[&(i, j)] {
            continue;
        }

        for (x, y) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if let Some(v) = array.get(y).and_then(|f| f.get(x)) {
                let alt = cost + v;
                let new = (alt, x, y);

                if alt < dist[&(x, y)] {
                    dist.insert((x, y), alt);
                    heap.push(new);
                }
            }
        }
    }

    unreachable!()
}

pub fn part1() -> usize {
    path(&(0, 0, 0), &(1, 99, 99), &parse_input()) as usize
}

pub fn part2() -> usize {
    let tile = parse_input();
    let mut bigger_tile: Vec<Vec<u32>> = vec![vec![]; tile.len()];

    for (i, d) in tile.iter().enumerate() {
        for j in 0..5 {
            let extend: Vec<_> = d
                .iter()
                .map(|f| if f + j > 9 { (f + j) % 9 } else { f + j })
                .collect();
            extend.iter().for_each(|f| bigger_tile[i].push(*f));
        }
    }

    let clone = bigger_tile.clone();
    for j in 1..5 {
        let extension: Vec<Vec<_>> = clone
            .iter()
            .map(|f| {
                f.iter()
                    .map(|n| if *n + j > 9 { (*n + j) % 9 } else { *n + j })
                    .collect()
            })
            .collect();
        for ex in extension {
            bigger_tile.push(ex);
        }
    }

    path(&(0, 0, 0), &(1, 499, 499), &bigger_tile) as usize
}
