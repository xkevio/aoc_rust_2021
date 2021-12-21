use colored::Colorize;
use regex::{Regex, Captures};

const INPUT: &str = include_str!("../inputs/day18.txt");

fn magnitude_of_snail_fish_num(num: &str) -> usize {
    println!("-----------------MAGNITUDE-----------------");
    dbg!(num);

    let regex = Regex::new(r"\[(\d+),(\d+)]").unwrap();
    let mut result = num.clone().to_string();

    loop {
        let new_result = regex.replace_all(&result, |c: &Captures| {
            let left: i32 = c[1].parse().unwrap();
            let right: i32 = c[2].parse().unwrap();
            
            format!("{}", 3 * left + 2 * right)
        }).to_string();

        if new_result == result {
            result = new_result;
            break;
        } else {
            result = new_result;
        }
    }

    result.parse().unwrap()
}

fn split_snail_fish_num(num: &str, n: &str) -> String {
    let mut split_num = num.clone().to_string();

    // println!("{}", "---------------SPLIT---------------".red().bold());
    // dbg!("SPLIT");

    let number_to_split: f32 = n.parse().unwrap();
    let left = (number_to_split / 2.0).floor() as i32;
    let right = (number_to_split / 2.0).ceil() as i32;

    let new_pair = format!("[{},{}]", left, right);

    split_num = split_num.replacen(n, &new_pair, 1);

    split_num
}

fn explode_snail_fish_num(num: &str, pair: &str, start: usize, end: usize) -> String {
    let nums: Vec<usize> = pair.split(|c: char| !c.is_numeric()).flat_map(|f| f.parse()).collect();

    // println!("{}", "---------------EXPLODE---------------".red().bold());
    // dbg!("EXPLODE");

    // dbg!(&nums);
    // dbg!(pair);

    let (a, b) = (nums[0], nums[1]);
    let mut exploded_num = num.clone().to_string();

    // go left and find number if any and add a
    let mut left_num = String::new();
    let mut left_index: usize = 0;

    for i in (0..start).rev() {
        if let Some(c) = exploded_num.chars().nth(i) {
            if c.is_numeric() {
                left_num.insert(0, c);
                left_index = i;
            } else {
                if !left_num.is_empty() {
                    break;
                }
            }
        }
    }

    // dbg!(&left_num);
    let mut offset = 0;

    if !left_num.is_empty() {
        let new_left_num: usize = a + left_num.parse::<usize>().unwrap();
        offset = if new_left_num.to_string().len() > left_num.len() { new_left_num.to_string().len() - 1} else { 0 };

        exploded_num.replace_range(left_index..(left_index + left_num.len()), &new_left_num.to_string());
    }
    
    // go right and find number if any and add b
    let mut right_num = String::new();
    let right_index: usize = end + 2 + offset;

    let mut range = 0;

    // dbg!(offset);

    for i in right_index..exploded_num.len() {
        if let Some(c) = exploded_num.chars().nth(i) {
            if c.is_numeric() {
                right_num.push(c);
                range = i;
            } else {
                if !right_num.is_empty() {
                    break;
                }
            }
        }
    }

    // dbg!(&right_num);

    if !right_num.is_empty() {
        let new_right_num: usize = b + right_num.parse::<usize>().unwrap();
        exploded_num.replace_range((range - right_num.len() + 1)..=range, &new_right_num.to_string());
    }

    // replace pair with "0"
    // dbg!(&exploded_num);
    exploded_num.replace_range(start + offset..=(exploded_num[start + offset..].find(']').unwrap() + (start + offset)), "0");
    // exploded_num = exploded_num.replacen(pair, "0", 1);

    // return
    exploded_num
}

fn reduce_snail_fish_num(num: &str) -> String {
    let mut pair_counter: usize = 0;
    let mut reduced_num = String::from(num);

    // dbg!(&reduced_num);

    let mut explode = false;

    for (i, c) in num.chars().enumerate() {
        match c {
            '[' => pair_counter += 1,
            ']' => pair_counter -= 1,
            _ => (),
        }

        // explosion
        if pair_counter > 4 {
            // explode, give start and end
            if explode && c.is_numeric() {
                // dbg!(i);

                let sub_string = &reduced_num[i-1..=(reduced_num[i-1..].find(']').unwrap() + (i - 1))];

                // dbg!(sub_string);

                let exploded_result = explode_snail_fish_num(&reduced_num, sub_string, i-1, reduced_num[i-1..].find(']').unwrap() + (i - 1));

                // dbg!(&exploded_result);

                //wrong order
                reduced_num = reduce_snail_fish_num(&exploded_result);
                break;
            }

            explode = true;
        }
    }

    // split
    let mut nums: Vec<&str> = reduced_num.split(|c: char| !c.is_numeric()).filter(|f| f.len() > 1).collect();
    if !nums.is_empty() {
        let n = nums.remove(0);
        let split_num = split_snail_fish_num(&reduced_num, n);

        reduced_num = reduce_snail_fish_num(&split_num); 
    }

    reduced_num
}

fn add_snail_fish_nums(a: &str, b: &str) -> String {
    println!("ADDING: {} + {}", a.bold().green(), b.bold().green());

    let result = format!("[{},{}]", a, b);
    let reduced_result = reduce_snail_fish_num(&result);

    dbg!(&result);
    dbg!(&reduced_result);

    reduced_result
}

pub fn part1() -> usize {
    // reduce might not work since the nums arent commutative
    let mut result: String = INPUT.lines().nth(0).unwrap().to_string();

    for line in INPUT.lines().skip(1) {
        result = add_snail_fish_nums(&result, line);
    }

    dbg!(&result);
    dbg!(magnitude_of_snail_fish_num(&result));

    magnitude_of_snail_fish_num(&result)
}

pub fn part2() -> usize {
    let nums: Vec<&str> = INPUT.lines().collect();
    let mut magnitudes: Vec<usize> = Vec::new();

    for n1 in &nums {
        for n2 in &nums {
            if n1 != n2 {
                let result = add_snail_fish_nums(n1, n2);
                magnitudes.push(magnitude_of_snail_fish_num(&result));
            }
        }
    }

    *magnitudes.iter().max().unwrap()
}