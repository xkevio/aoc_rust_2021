const INPUT: &str = include_str!("inputs/day4.txt");

struct Board {
    grid: Vec<Vec<i32>>,
    done: bool,
}

impl Board {
    fn new() -> Self {
        Board { grid: vec![], done: false }
    }

    fn add_row(&mut self, row: Vec<i32>) {
        self.grid.push(row);
    }

    fn contains(&self, num: i32) -> bool {
        for x in &self.grid {
            for y in x {
                if *y == num {
                    return true;
                }
            }
        }

        false
    }

    fn mark(&mut self, num: i32) {
        for x in &mut self.grid {
            if let Some(i) = x.iter().position(|n| *n == num) {
                x[i] = -1
            }
        }
    }

    fn sum_positive(&self) -> i32 {
        let mut sum = 0;

        for x in &self.grid {
            sum += x.iter().filter(|n| **n >= 0).sum::<i32>();
        }

        sum
    }

    fn check(&self) -> bool {
        let mut row = false;
        let mut col = false;

        for x in &self.grid {
            row = row || x.into_iter().all(|v| *v == -1);
        }

        for i in 0..self.grid[0].len() {
            let mut column_b = true;
            for j in 0..self.grid[0].len() {
                column_b = column_b && (self.grid[j][i] == -1);
            }

            col = col || column_b;
        }

        row || col
    }
}

fn get_boards() -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();

    for board_s in INPUT.split("\n\n").skip(1) {
        let mut board = Board::new();
        let t = board_s
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        for r in t {
            board.add_row(r);
        }

        boards.push(board);
    }

    boards
}

pub fn part1() -> i32 {
    let instructions: String = INPUT.split("\n\n").take(1).collect();
    let mut boards = get_boards();

    let result = 0;

    for ins in instructions.split(",") {
        let num: i32 = ins.parse().unwrap();

        for board in &mut boards {
            if board.contains(num) {
                board.mark(num);
                if board.check() {
                    println!("{} * {}", &num, board.sum_positive());
                    return num * board.sum_positive();
                }
            }
        }
    }

    result
}

pub fn part2() -> i32 {
    let instructions: String = INPUT.split("\n\n").take(1).collect();
    let mut boards = get_boards();

    let mut result = 0;

    for ins in instructions.split(",") {
        let num: i32 = ins.parse().unwrap();

        for board in &mut boards {

            if board.done {
                continue;
            }

            if board.contains(num) {
                board.mark(num);
                if board.check() {
                    board.done = true;
                    println!("{} * {}", &num, board.sum_positive());
                    result = num * board.sum_positive();
                }
            }
        }
    }

    result
}
