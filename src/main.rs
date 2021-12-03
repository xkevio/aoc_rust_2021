use std::error::Error;

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", day2::part1()?);
    println!("Part 2: {}", day2::part2()?);

    Ok(())
}
