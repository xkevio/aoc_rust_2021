use std::error::Error;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", day3::part1());
    println!("Part 2: {}", day3::part2());

    Ok(())
}
