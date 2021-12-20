const INPUT: &str = include_str!("../inputs/day20.txt");

type Grid = Vec<Vec<char>>;

fn pad_grid(grid: &mut Grid, pad: char) {
    grid.insert(0, vec![pad; grid[0].len()]);
    grid.push(vec![pad; grid[0].len()]);

    for v in grid {
        v.insert(0, pad);
        v.push(pad);
    }
}

fn process_neighbors(map: &Grid, i: usize, j: usize) -> usize {
    let mut number = String::new();

    for (x, y) in [(i - 1, j - 1), (i - 1, j), (i - 1, j + 1), (i, j - 1), (i, j), (i, j + 1), (i + 1, j - 1), (i + 1, j), (i + 1, j + 1)] {
        if let Some(v) = map.get(x).and_then(|f| f.get(y)) {
            number.push(if *v == '.' { '0' } else { '1' });
        }
    }

    usize::from_str_radix(number.as_str(), 2).unwrap()
}

fn enhance(times: usize) -> usize {
    let (algo, map) = INPUT.split_once("\r\n\r\n").unwrap();
    let mut grid: Grid = Vec::new();

    let algo_vec = algo.chars().collect::<Vec<_>>();

    for line in map.lines() {
        let char_vec: Vec<char> = line.chars().collect();
        grid.push(char_vec);
    }

    pad_grid(&mut grid, '.');

    for _ in 0..times {
        let c = grid[0][0];

        pad_grid(&mut grid, c);
        let mut temp_map = grid.clone();

        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                let index = process_neighbors(&grid, i, j);
                let new_char = algo_vec[index];

                temp_map[i][j] = new_char;
            }
        }
        
        let length = temp_map.len();
        let vc = if c == '#' { algo_vec[511] } else { algo_vec[0] };

        temp_map[0] = vec![vc; temp_map[0].len()];
        temp_map[length - 1] = vec![vc; temp_map[0].len()];

        for v in &mut temp_map {
            let v_len = v.len();

            v[0] = vc;
            v[v_len - 1] = vc;
        }

        grid = temp_map.clone();
    }

    grid.iter().map(|f| f.iter().filter(|c| **c == '#').count()).sum()
}

pub fn part1() -> usize {
    enhance(2)
}

pub fn part2() -> usize {
    enhance(50)
}