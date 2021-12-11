const INPUT: &str = include_str!("../inputs/day6.txt");

fn lantern_fish_growth(days: usize) -> usize {
    let lantern_fish: Vec<usize> = INPUT.split(',').flat_map(|a| a.parse()).collect();
    let mut length = [0; 9];

    for x in &lantern_fish {
        length[*x] += 1;
    }

    for _ in 0..days {
        length.rotate_left(1);
        length[6] += length[8];
    }

    length.iter().sum()
}

pub fn part1() -> usize {
    lantern_fish_growth(80)
}

pub fn part2() -> usize {
    lantern_fish_growth(256)
}
