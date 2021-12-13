const INPUT: &str = include_str!("../inputs/day13.txt");

type Matrix = Vec<Vec<i32>>;

fn print_matrix(matrix: &Matrix) {
    for x in matrix {
        for y in x {
            print!("{}", if *y < 0 { '█' } else { ' ' });
        }
        println!();
    }
}

fn parse_input() -> Matrix {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; 1311]; 895];
    INPUT
        .split_once("\r\n\r\n")
        .unwrap()
        .0
        .lines()
        .flat_map(|l| l.split_once(','))
        .map(|p| (p.0.parse::<usize>().unwrap(), p.1.parse::<usize>().unwrap()))
        .for_each(|p| matrix[p.1][p.0] = -1);
    matrix
}

fn fold(pos: usize, axis: char, matrix: &Matrix) -> Matrix {
    let mut folded_matrix: Matrix = Vec::new();
    let mut folder: Matrix = matrix.clone();

    if axis == 'y' {
        for i in 0..pos {
            folded_matrix.push(matrix[i].clone());
        }
        folder.drain(0..=pos);

        for i in 0..folded_matrix.len() {
            for j in 0..folded_matrix[0].len() {
                folded_matrix[i][j] += folder[folder.len() - 1 - i][j];
            }
        }
    }
    if axis == 'x' {
        for i in 0..matrix.len() {
            let mut v: Vec<i32> = Vec::new();
            for j in 0..pos {
                v.push(matrix[i][j]);
            }

            folder[i].drain(0..=pos);
            folded_matrix.push(v);
        }

        for i in 0..folded_matrix.len() {
            for j in 0..folded_matrix[0].len() {
                folded_matrix[i][j] += folder[i][folder[0].len() - 1 - j];
            }
        }
    }

    folded_matrix
}

fn fold_n(fold_once: bool) -> usize {
    let instructions = INPUT.split_once("\r\n\r\n").unwrap().1;
    let mut matrix = parse_input();
    let mut dots: usize = 0;

    for ins in instructions.lines() {
        let (axis, pos) = ins.split_once('=').unwrap();
        let num_axis = axis.replace("fold along ", "").parse::<char>().unwrap();
        let num_pos = pos.parse::<usize>().unwrap();

        matrix = fold(num_pos, num_axis, &matrix);

        dots += matrix
            .iter()
            .map(|v| v.iter().filter(|n| **n < 0).count())
            .sum::<usize>();

        if fold_once {
            break;
        }
    }

    if !fold_once {
        print_matrix(&matrix);
        println!(); 
    }

    dots
}

pub fn part1() -> usize {
    fold_n(true)
}

pub fn part2() -> usize {
    fold_n(false)
}
