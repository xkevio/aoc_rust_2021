const INPUT: &str = include_str!("inputs/day1.txt");

// pub fn part1_better() -> usize {
//     let mut vec = Vec::<usize>::new();
//     let measurements: Vec<usize> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

//     for i in 1..measurements.len() {
//         vec.push(measurements[i] - measurements[i - 1]);
//     }

//     vec.iter().filter(|x| **x > 0).count()
// }

pub fn part1() -> u32 {
    let mut increases: u32 = 0;

    let measurements: Vec<u32> = INPUT.lines().map(|n| n.parse().unwrap()).collect();

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }

    increases
}

pub fn part2() -> u32 {
    let mut increases: u32 = 0;

    let measurements: Vec<u32> = INPUT.lines().map(|n| n.parse().unwrap()).collect();
    let windows = measurements.windows(3).collect::<Vec<_>>();

    for i in 1..windows.len() {
        if windows[i].iter().sum::<u32>() > windows[i - 1].iter().sum() {
            increases += 1;
        }
    }

    increases
}
