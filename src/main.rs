use std::{error::Error, time::Instant};

#[allow(dead_code)]
mod days;

fn main() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();
    println!("Part 1: {}", days::day11::part1());
    println!("Part 2: {}", days::day11::part2());
    let end = Instant::now() - time;

    println!("time: {}µs", end.as_micros());

    Ok(())
}
