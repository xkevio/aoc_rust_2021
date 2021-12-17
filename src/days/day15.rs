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

fn get_neighbors(i: usize, j: usize, input: &[Vec<u32>]) -> Vec<Node> {
    let mut neighbors: Vec<Node> = Vec::new();

    // has to be run with --release because of panic! for underflow (could use wrapping_sub)
    for (y, x) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
        if let Some(v) = input.get(y).and_then(|f| f.get(x)) {
            neighbors.push((*v, y, x));
        }
    }

    neighbors
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

    while let Some(u) = heap.pop_min() {
        if goal.1 == u.1 && goal.2 == u.2 {
            return u.0;
        }

        if u.0 > dist[&(u.1, u.2)] {
            continue;
        }

        for v in &get_neighbors(u.1, u.2, array) {
            let alt = u.0 + v.0;
            let new = (alt, v.1, v.2);

            if alt < dist[&(v.1, v.2)] {
                dist.insert((v.1, v.2), alt);
                heap.push(new);
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
