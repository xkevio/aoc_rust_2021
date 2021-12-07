use std::cmp::{max, min};

const INPUT: &str = include_str!("inputs/day7.txt");

fn get_median(list: &[u32]) -> u32 {
    if list.is_empty() {
        return 0;
    }

    let mid = list.len() / 2;
    
    if list.len() % 2 == 1 {
        return list[mid];
    } else {
        return (list[mid - 1] + list[mid]) / 2;
    }
}

pub fn part1() -> u32 {
    let mut coordinates: Vec<u32> = INPUT.split(",").map(|f| f.parse().unwrap()).collect();
    coordinates.sort();

    let pos = get_median(&coordinates);
    coordinates.iter().map(|f| max(*f, pos) - min(*f, pos)).sum()
}

pub fn part2() -> u32 {
    let coordinates: Vec<u32> = INPUT.split(",").map(|f| f.parse().unwrap()).collect();
    let pos = f32::ceil((coordinates.iter().sum::<u32>() / coordinates.len() as u32) as f32) as u32;

    coordinates.iter().map(|f| max(*f, pos) - min(*f, pos)).map(|f| (f * (f + 1)) / 2).sum()
}