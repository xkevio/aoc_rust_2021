const INPUT: &str = include_str!("inputs/day2.txt");

#[allow(dead_code)]
#[rustfmt::skip]
pub fn part1_iterator() -> usize {
    let horizontal: usize = INPUT.lines().filter(|l| l.starts_with("f")).map(|l| l.split_once(" ").unwrap().1.parse::<usize>().unwrap()).sum();

    let mut depth: usize = INPUT.lines().filter(|l| l.starts_with("d")).map(|l| l.split_once(" ").unwrap().1.parse::<usize>().unwrap()).sum();
    depth -= INPUT.lines().filter(|l| l.starts_with("u")).map(|l| l.split_once(" ").unwrap().1.parse::<usize>().unwrap()).sum::<usize>();

    horizontal * depth
}

pub fn part1() -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for l in INPUT.lines() {
        let (op, v) = l.split_once(" ").unwrap();
        match op {
            "forward" => horizontal += v.parse::<i32>().unwrap(),
            "down" => depth += v.parse::<i32>().unwrap(),
            "up" => depth -= v.parse::<i32>().unwrap(),
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
        let (op, v) = l.split_once(" ").unwrap();
        match op {
            "forward" => {
                horizontal += v.parse::<i32>().unwrap();
                depth += aim * v.parse::<i32>().unwrap();
            }
            "down" => aim += v.parse::<i32>().unwrap(),
            "up" => aim -= v.parse::<i32>().unwrap(),
            _ => println!("error"),
        }
    }

    horizontal * depth
}
