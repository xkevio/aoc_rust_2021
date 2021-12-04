const INPUT: &str = include_str!("inputs/day4.txt");

type Board = Vec<Vec<i32>>;

fn board_contains(board: &Board, value: i32) -> Option<(usize, usize)> {
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if board[x][y] == value {
                return Some((x, y));
            }
        }
    }
    None
}

fn bingo(board: &Board) -> bool {
    let row = board.iter().any(|r| r.iter().all(|n| *n == -1));
    let col = (0..5).any(|i| board.iter().all(|c| c[i] == -1));

    row || col
}

fn get_boards() -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();

    for board_s in INPUT.split("\n\n").skip(1) {
        let board = board_s
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        boards.push(board);
    }

    boards
}

fn emulate_bingo(part1: bool) -> i32 {
    let instructions: String = INPUT.lines().take(1).collect();
    let mut boards = get_boards();

    let mut result = 0;

    for ins in instructions.split(",") {
        let num: i32 = ins.parse().unwrap();

        for board in &mut boards {
            if let Some((x, y)) = board_contains(board, num) {
                board[x][y] = -1;
            }
            if bingo(board) && !board.is_empty() {
                let sum: i32 = board
                    .iter()
                    .map(|r| r.iter().filter(|i| **i >= 0).sum::<i32>())
                    .sum();

                if part1 {
                    return num * sum;
                } else {
                    result = num * sum;
                }

                board.clear();
            }
        }
    }

    result
}

pub fn part1() -> i32 {
    emulate_bingo(true)
}

pub fn part2() -> i32 {
    emulate_bingo(false)
}
