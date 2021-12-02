const INPUT: &str = include_str!("inputs/day2.txt");

pub fn part1_iterator() -> usize {
    let horizontal: usize = INPUT.lines().filter(|l| l.starts_with("f")).map(|l| l.strip_prefix("forward ").unwrap().parse::<usize>().unwrap()).sum();
    let mut depth: usize = INPUT.lines().filter(|l| l.starts_with("d")).map(|l| l.strip_prefix("down ").unwrap().parse::<usize>().unwrap()).sum();
    depth -= INPUT.lines().filter(|l| l.starts_with("u")).map(|l| l.strip_prefix("up ").unwrap().parse::<usize>().unwrap()).sum::<usize>();

    horizontal * depth
}

pub fn part1() -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for l in INPUT.lines() {
        let values: Vec<&str> = l.split(" ").collect();
        match values[0] {
            "forward" => horizontal += values[1].parse::<i32>().unwrap(),
            "down" => depth += values[1].parse::<i32>().unwrap(),
            "up" => depth -= values[1].parse::<i32>().unwrap(),
            _ => println!("error"),
        }
    } 

    horizontal * depth
}

pub fn part2() -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in INPUT.lines() {
        let values: Vec<&str> = l.split(" ").collect();
        match values[0] {
            "forward" => {
                horizontal += values[1].parse::<i32>().unwrap();
                depth += aim * values[1].parse::<i32>().unwrap();
            },
            "down" => aim += values[1].parse::<i32>().unwrap(),
            "up" => aim -= values[1].parse::<i32>().unwrap(),
            _ => println!("error"),
        }
    } 

    horizontal * depth
}

