use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day22.txt");

pub fn part1() -> usize {
    let mut grid: HashMap<(i64, i64, i64), bool> = HashMap::new();

    for line in INPUT.lines() {
        let nums: Vec<i64> = line
            .split(|c: char| if c != '-' { !c.is_numeric() } else { false })
            .flat_map(|f| f.parse())
            .collect();

        for x in nums[0]..=nums[1] {
            for y in nums[2]..=nums[3] {
                for z in nums[4]..=nums[5] {
                    // if (0..=100).contains(&x) && (0..=100).contains(&y) && (0..=100).contains(&z) {
                        // let num = if line.starts_with("on") { 1 } else { 0 };
                        grid.insert((x, y, z), line.starts_with("on"));
                    // } else {
                        // break 'o;
                    // }
                }
            }
        }
    }
    
    dbg!(grid.keys().len());
    grid.values().filter(|b| **b).count()
}

pub fn part2() -> usize {
    0
}