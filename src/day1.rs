use std::{fs, u32::MAX};

pub fn part1() -> u32 {
    let mut increases: u32 = 0;
    let mut previous_measurement: u32 = MAX;

    let sonar_measurements =
        fs::read_to_string("src/inputs/day1.txt").expect("Unable to open file");

    for measurement in sonar_measurements.lines().collect::<Vec<_>>() {
        let current_measurement: u32 = measurement.parse().unwrap();
        if current_measurement > previous_measurement {
            increases += 1;
        }
        previous_measurement = current_measurement;
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
