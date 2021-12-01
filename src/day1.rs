use std::fs;

// pub fn part1_better() -> usize {
//     let sonar_measurements =
//         fs::read_to_string("src/inputs/day1.txt").expect("Unable to open file");

//     let mut vec = Vec::<i32>::new();
//     let measurements: Vec<&str> = sonar_measurements.lines().collect();

//     for i in 1..measurements.len() {
//         vec.push(measurements[i].parse::<i32>().unwrap() - measurements[i - 1].parse::<i32>().unwrap());
//     }

//     vec.iter().filter(|x| **x > 0).count()
// }

pub fn part1() -> u32 {
    let mut increases: u32 = 0;

    let sonar_measurements =
        fs::read_to_string("src/inputs/day1.txt").expect("Unable to open file");

    let measurements: Vec<u32> = sonar_measurements
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }

    increases
}

pub fn part2() -> u32 {
    let mut increases: u32 = 0;
    let sonar_measurements =
        fs::read_to_string("src/inputs/day1.txt").expect("Unable to open file");

    let measurements: Vec<u32> = sonar_measurements
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let windows = measurements.windows(3).collect::<Vec<_>>();

    for i in 1..windows.len() {
        if windows[i].iter().sum::<u32>() > windows[i - 1].iter().sum() {
            increases += 1;
        }
    }

    increases
}
