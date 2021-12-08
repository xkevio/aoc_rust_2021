use std::{error::Error, time::Instant};

// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;

mod day8;

fn main() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();
    println!("Part 1: {}", day8::part1());
    println!("Part 2: {}", day8::part2());
    let end = Instant::now() - time;

    println!("time: {}µs", end.as_micros());

    Ok(())
}
