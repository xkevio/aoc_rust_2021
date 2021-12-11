const INPUT: &str = include_str!("../inputs/day3.txt");

pub fn part1() -> u32 {
    let mut gamma_rate = 0;

    for i in 0..12 {
        let (ones, zeros): (Vec<&str>, Vec<&str>) =
            INPUT.lines().partition(|l| l[i..l.len()].starts_with('1'));

        if ones.len() > zeros.len() {
            gamma_rate += 2u32.pow(11 - i as u32);
        }
    }

    // "flip bits" / invert
    let epsilon = 2u32.pow(12) - 1 - gamma_rate;
    gamma_rate * epsilon
}

pub fn part2() -> u32 {
    let nums: Vec<&str> = INPUT.lines().collect();

    let o2_gen_rating = reduce_to_rating(&nums, true);
    let co2_scrubber_rating = reduce_to_rating(&nums, false);

    o2_gen_rating * co2_scrubber_rating
}

#[rustfmt::skip]
fn reduce_to_rating(numbers: &[&str], start_bit: bool) -> u32 {
    let mut i = 0;
    let mut numbers_copy = numbers.to_owned();

    while numbers_copy.len() > 1 {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = numbers_copy.iter().partition(|l| l[i..l.len()].starts_with('1'));

        if ones.len() >= zeros.len() {
            numbers_copy.retain(|n| if start_bit { ones.contains(n) } else { zeros.contains(n) });
        } else {
            numbers_copy.retain(|n| if start_bit { zeros.contains(n) } else { ones.contains(n) });
        }

        i += 1;
    }

    u32::from_str_radix(numbers_copy[0], 2).unwrap()
}
