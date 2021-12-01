use std::{fs, u32::MAX};

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

    let measurements: Vec<u32> = sonar_measurements.lines().map(|n| n.parse::<u32>().unwrap()).collect();

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i-1] {
            increases += 1;
        }
    }
    
    increases
}

pub fn part2() -> u32 {
    let mut increases: u32 = 0;
    let sonar_measurements =
        fs::read_to_string("src/inputs/day1.txt").expect("Unable to open file");

    let lines = sonar_measurements.lines().collect::<Vec<_>>();
    let windows = lines.windows(3);

    let mut previous_measurement: u32 = MAX;

    for w in windows {
        let num_w: Vec<u32> = w
            .iter()
            .map(|window| window.parse().expect("Parsing error"))
            .collect();
        let current_measurement: u32 = num_w.iter().sum();

        if current_measurement > previous_measurement {
            increases += 1;
        }

        previous_measurement = current_measurement;
    }

    increases
}
