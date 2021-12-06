const INPUT: &str = include_str!("inputs/day6.txt");

fn lantern_fish_growth(days: usize) -> usize {
    let lantern_fish: Vec<usize> = INPUT.split(",").map(|a| a.parse().unwrap()).collect();
    let mut length = [0; 9];

    for x in &lantern_fish {
        length[*x] += 1;
    }

    // println!("initial state: {:?}", &length);

    for _ in 0..days {
        let mut copy = length.clone();

        for i in 0..length.len() {
            if length[i] > 0 {
                match i {
                    0 => {
                        for _ in 0..length[i] {
                            copy[i] -= 1;
                            copy[6] += 1;
                            copy[8] += 1; 
                        }
                    },
                    _ => {
                        for _ in 0..length[i] {
                            copy[i] -= 1;
                            copy[i-1] += 1; 
                        }
                        
                    },
                }

            }
        }

        length = copy.clone();
        // println!("Day {}: {:?}", d+1, &length);
    }

    length.iter().sum()
}

pub fn part1() -> usize {
    lantern_fish_growth(80)
}

pub fn part2() -> usize {
    lantern_fish_growth(256)
}