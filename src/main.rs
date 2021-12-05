use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", day5::part1());
    println!("Part 2: {}", day5::part2());

    Ok(())
}
