const INPUT: &str = include_str!("../inputs/day25.txt");

fn parse_input() -> Vec<Vec<char>> {
    INPUT.lines().map(|l| l.chars().collect()).collect()
}

fn get_move(grid: &[Vec<char>], i: usize, j: usize, c: char) -> Option<(usize, usize)> {
    match c {
        '>' => {
            if (j + 1) < grid[0].len() {
                if grid[i][j + 1] == '.' {
                    Some((i, j + 1))
                } else {
                    None
                }
            } else {
                if grid[i][0] == '.' {
                    Some((i, 0))
                } else {
                    None
                }
            }
        },
        'v' => {
            if (i + 1) < grid.len() {
                if grid[i + 1][j] == '.' {
                    Some((i + 1, j))
                } else {
                    None
                }
            } else {
                if grid[0][j] == '.' {
                    Some((0, j))
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

pub fn part1() -> usize {
    let mut grid = parse_input();
    let mut steps: usize = 0;

    for step in 1.. {
        let mut moved = false;
        
        for c in ['>', 'v'] {
            let mut next = grid.clone();
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == c {
                        if let Some(p) = get_move(&grid, i, j, c) {
                            next[i][j] = '.';
                            next[p.0][p.1] = c;

                            moved = true;
                        }
                    }
                }
            }
            grid = next;
        }
        
        if !moved {
            steps = step;
            break;
        }
    }

    steps
}

pub fn part2() -> usize {
    println!("No part 2 for today!");
    0
}
