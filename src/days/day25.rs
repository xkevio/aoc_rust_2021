use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day25.txt");

enum Direction {
    Right,
    Down,
}

fn parse_input() -> Vec<Vec<char>> {
    INPUT.lines().map(|l| l.chars().collect()).collect()
}

fn get_move(grid: &[Vec<char>], i: usize, j: usize, dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::Right => {
            if (j + 1) < grid[0].len() && grid[i][j + 1] == '.' {
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
        Direction::Down => {
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
    }
}

pub fn part1() -> usize {
    let mut grid = parse_input();
    let mut steps: usize = 0;

    for step in 1.. {
        let mut moves: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut no_right_moves = false;
        let mut no_down_moves = false;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '>' {
                    if let Some(p) = get_move(&grid, i, j, Direction::Right) {
                        moves.insert((i, j), p);
                    }
                }
            }
        }

        for (from, to) in &moves {
            grid[from.0][from.1] = '.';
            grid[to.0][to.1] = '>';
        }

        if moves.is_empty() {
            no_right_moves = true;
        }

        moves.clear();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'v' {
                    if let Some(p) = get_move(&grid, i, j, Direction::Down) {
                        moves.insert((i, j), p);
                    }
                }
            }
        }

        for (from, to) in &moves {
            grid[from.0][from.1] = '.';
            grid[to.0][to.1] = 'v';
        }

        if moves.is_empty() {
            no_down_moves = true;
        }

        if no_right_moves && no_down_moves {
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
