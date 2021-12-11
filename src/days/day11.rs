const INPUT: &str = include_str!("../inputs/day11.txt");

#[rustfmt::skip]
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

    let dpos: Vec<(i32, i32)> = (-1..=1)
        .map(|dx| (-1..=1).map(|dy| (dx, dy)).collect::<Vec<_>>())
        .flatten()
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
        .collect();

    for (dx, dy) in dpos {
        let (x, y) = (dx + i as i32, dy + j as i32);
        if x >= 0 && x < energy_levels.len() as i32 && y >= 0 && y < energy_levels[0].len() as i32 {
            neighbors.push((x as usize, y as usize));
        }
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

        //increase by 1
        energy_levels = energy_levels
            .iter()
            .map(|v| v.iter().map(|n| *n + 1).collect())
            .collect();

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
        energy_levels = energy_levels
            .iter()
            .map(|v| v.iter().map(|n| if *n > 9 { 0 } else { *n }).collect())
            .collect();

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
