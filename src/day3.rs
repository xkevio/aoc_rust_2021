const INPUT: &str = include_str!("inputs/day3.txt");

pub fn part1() -> u32 {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..12 {
        let one_count = INPUT.lines().filter(|l| l[i..l.len()].starts_with("1")).count();
        let zero_count = INPUT.lines().filter(|l| l[i..l.len()].starts_with("0")).count();

        if one_count > zero_count {
            gamma_rate += "1";
        } else {
            gamma_rate += "0";
        }
    }

    // "flip bits"
    epsilon_rate = gamma_rate.as_str().chars().map(|c| if c == '1' { '0' } else { '1' }).collect();
    u32::from_str_radix(gamma_rate.as_str(), 2).unwrap() * u32::from_str_radix(epsilon_rate.as_str(), 2).unwrap()
}

pub fn part2() -> u32 {
    let mut o2_gen_rating = 0;
    let mut co2_scrubber_rating = 0;

    let nums: Vec<&str> = INPUT.lines().collect();

    o2_gen_rating = retain_ml(&nums, true);
    co2_scrubber_rating = retain_ml(&nums, false);

    println!("{}, {}", &o2_gen_rating, &co2_scrubber_rating);

    o2_gen_rating * co2_scrubber_rating
}

fn retain_ml(numbers: &Vec<&str>, most_common: bool) -> u32 {
    let mut i = 0;
    let mut numbers_copy = numbers.clone();

    while numbers_copy.len() > 1 {
        let mut one_count = 0;
        let mut zero_count = 0;

        for n in &numbers_copy {
            if n[i..n.len()].starts_with("1") {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }

        if one_count >= zero_count {
            numbers_copy.retain(|n| n[i..n.len()].starts_with((most_common as u32).to_string().as_str()));  
        } else {
            numbers_copy.retain(|n| n[i..n.len()].starts_with((!most_common as u32).to_string().as_str()));  
        }

        i += 1;
    }

    u32::from_str_radix(numbers_copy[0], 2).unwrap()
}