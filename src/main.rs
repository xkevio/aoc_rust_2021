use std::{error::Error, time::Instant};

#[allow(dead_code)]
mod days;

fn main() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();
    println!("Part 1: {}", days::day17::part1());
    println!("Part 2: {}", days::day17::part2());
    let end = time.elapsed();

    println!("time: {}ms", end.as_millis());

    Ok(())
}
