const INPUT: &str = include_str!("../inputs/day7.txt");

pub fn part1() -> usize {
    let coordinates: Vec<i64> = INPUT.split(',').flat_map(|f| f.parse()).collect();
    let pos = statistical::median(&coordinates);

    coordinates.iter().map(|f| (f - pos).abs() as usize).sum()
}

pub fn part2() -> usize {
    let coordinates: Vec<f64> = INPUT.split(',').flat_map(|f| f.parse()).collect();
    let pos = f64::floor(statistical::mean(&coordinates)); // for other inputs, ceil()

    coordinates.iter().map(|f| (f - pos).abs() as usize).map(|f| ((f * (f + 1)) / 2)).sum()
}
