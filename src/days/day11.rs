const INPUT: &str = include_str!("../inputs/day11.txt");

fn parse_input() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_neighbors(energy_levels: &[Vec<u32>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    // cardinal
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if i < energy_levels.len() - 1 {
        neighbors.push((i + 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if j < energy_levels[0].len() - 1 {
        neighbors.push((i, j + 1));
    }

    // diagonal
    if i > 0 && j > 0 {
        neighbors.push((i - 1, j - 1));
    }
    if i < energy_levels.len() - 1 && j < energy_levels[0].len() - 1 {
        neighbors.push((i + 1, j + 1));
    }
    if i < energy_levels.len() - 1 && j > 0 {
        neighbors.push((i + 1, j - 1));
    }
    if i > 0 && j < energy_levels[0].len() - 1 {
        neighbors.push((i - 1, j + 1));
    }

    neighbors
}

fn flash(energy_levels: &mut [Vec<u32>], i: usize, j: usize, marked: &mut Vec<(usize, usize)>) {
    if energy_levels[i][j] > 9 && !marked.contains(&(i, j)) {
        marked.push((i, j));
        for n in get_neighbors(energy_levels, i, j) {
            energy_levels[n.0][n.1] += 1;
            flash(energy_levels, n.0, n.1, marked);
        }
    }
}

fn flash_octopus(part1: bool) -> usize {
    let mut energy_levels = parse_input();
    let mut flashes: usize = 0;
    let mut moment: usize = 0;

    loop {
        moment += 1;
        if part1 && moment > 100 {
            break;
        }
        // increase by 1
        for x in &mut energy_levels {
            for y in x {
                *y += 1;
            }
        }

        // increase neighbors
        let mut marked: Vec<(usize, usize)> = Vec::new();

        for i in 0..energy_levels.len() {
            for j in 0..energy_levels[0].len() {
                if energy_levels[i][j] > 9 {
                    flash(&mut energy_levels, i, j, &mut marked);
                }
            }
        }

        // flashes
        flashes += energy_levels
            .iter()
            .map(|v| v.iter().filter(|n| **n > 9).count())
            .sum::<usize>();

        // reset
        for x in &mut energy_levels {
            for y in x {
                if *y > 9 {
                    *y = 0;
                }
            }
        }

        if !part1 && energy_levels.iter().all(|v| v.iter().all(|f| *f == 0)) {
            return moment;
        }
    }

    flashes
}

pub fn part1() -> usize {
    flash_octopus(true)
}

pub fn part2() -> usize {
    flash_octopus(false)
}
