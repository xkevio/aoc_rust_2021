const INPUT: &str = include_str!("../inputs/day1.txt");

fn parse_input() -> Vec<u32> {
    INPUT.lines().flat_map(|n| n.parse()).collect()
}

pub fn part1() -> u32 {
    let mut increases: u32 = 0;
    let measurements = parse_input();

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }

    increases
}

pub fn part2() -> u32 {
    let mut increases: u32 = 0;

    let measurements = parse_input();
    let windows = measurements.windows(3).collect::<Vec<_>>();

    for i in 1..windows.len() {
        if windows[i].iter().sum::<u32>() > windows[i - 1].iter().sum() {
            increases += 1;
        }
    }

    increases
}
