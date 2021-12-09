const INPUT: &str = include_str!("inputs/day9.txt");

fn parse_points() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| {
            l.chars()
            .flat_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()})
        .collect()
}

fn get_low_points(heightmap: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();

    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            if i > 0 && heightmap[i][j] >= heightmap[i - 1][j] {
                continue;
            }
            if i < heightmap.len() - 1 && heightmap[i][j] >= heightmap[i + 1][j] {
                continue;
            }
            if j > 0 && heightmap[i][j] >= heightmap[i][j - 1] {
                continue;
            }
            if j < heightmap[0].len() - 1 && heightmap[i][j] >= heightmap[i][j + 1] {
                continue;
            }

            points.push((i, j));
        }
    }

    points
}

fn flood_fill(i: usize, j: usize, heightmap: &[Vec<u32>], marked: &mut Vec<(usize, usize)>) -> u32 {
    let mut basin_size = 0u32;

    if heightmap[i][j] != 9 && ((i > 0 && j > 0) && (i < heightmap.len() - 1 && j < heightmap[0].len() - 1)) && !marked.contains(&(i, j)) {
        marked.push((i, j));
        basin_size += 1;

        basin_size += flood_fill(i + 1, j, heightmap, marked);
        basin_size += flood_fill(i, j + 1, heightmap, marked);
        basin_size += flood_fill(i - 1, j, heightmap, marked);
        basin_size += flood_fill(i, j - 1, heightmap, marked);
    }

    basin_size
}

pub fn part1() -> u32 {
    let heightmap = parse_points();
    get_low_points(&heightmap)
        .iter()
        .map(|c| heightmap[c.0][c.1] + 1)
        .sum()
}

pub fn part2() -> usize {
    let heightmap = parse_points();
    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut marked: Vec<(usize, usize)> = Vec::new();

    for l in get_low_points(&heightmap) {
        // flood fill, 9 is stop
        basin_sizes.push(flood_fill(l.0, l.1, &heightmap, &mut marked) as usize);
        marked.clear();
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}
