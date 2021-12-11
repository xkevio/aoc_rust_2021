use std::num::ParseIntError;

const INPUT: &str = include_str!("../inputs/day2.txt");

pub fn part1() -> Result<u32, ParseIntError> {
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

pub fn part2() -> Result<u32, ParseIntError> {
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
