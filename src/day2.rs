use std::str::FromStr;

const INPUT: &str = include_str!("inputs/day2.txt");

// #[allow(dead_code)]
// #[rustfmt::skip]
// pub fn part1_iterator() -> usize {
//     let horizontal: usize = INPUT.lines().filter(|l| l.starts_with("f")).map(|l| l.split_once(" ").unwrap().1.parse::<usize>().unwrap()).sum();

//     let mut depth: usize = INPUT.lines().filter(|l| l.starts_with("d")).map(|l| l.split_once(" ").unwrap().1.parse::<usize>().unwrap()).sum();
//     depth -= INPUT.lines().filter(|l| l.starts_with("u")).map(|l| l.split_once(" ").unwrap().1.parse::<usize>().unwrap()).sum::<usize>();

//     horizontal * depth
// }

pub fn part1() -> Result<u32, <u32 as FromStr>::Err> {
    let mut horizontal = 0;
    let mut depth = 0;

    for l in INPUT.lines() {
        let (op, v) = l.split_once(" ").unwrap();
        match op {
            "forward" => horizontal += v.parse::<u32>()?,
            "down" => depth += v.parse::<u32>()?,
            "up" => depth -= v.parse::<u32>()?,
            _ => println!("error"),
        }
    }

    Ok(horizontal * depth)
}

pub fn part2() -> Result<u32, <u32 as FromStr>::Err> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in INPUT.lines() {
        let (op, v) = l.split_once(" ").unwrap();
        match op {
            "forward" => {
                horizontal += v.parse::<u32>()?;
                depth += aim * v.parse::<u32>()?;
            }
            "down" => aim += v.parse::<u32>()?,
            "up" => aim -= v.parse::<u32>()?,
            _ => println!("error"),
        }
    }

    Ok(horizontal * depth)
}
