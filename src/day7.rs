const INPUT: &str = include_str!("inputs/day7.txt");

// fn get_median(list: &[usize]) -> usize {
//     if list.is_empty() {
//         return 0;
//     }

//     let mid = list.len() / 2;
    
//     if list.len() % 2 == 1 {
//         return list[mid];
//     } else {
//         return (list[mid - 1] + list[mid]) / 2;
//     }
// }

pub fn part1() -> usize {
    let mut coordinates: Vec<i64> = INPUT.split(",").map(|f| f.parse().unwrap()).collect();
    coordinates.sort();

    let pos = statistical::median(&coordinates);
    coordinates.iter().map(|f| (f - pos).abs() as usize).sum()
}

pub fn part2() -> usize {
    let coordinates: Vec<f64> = INPUT.split(",").map(|f| f.parse().unwrap()).collect();
    let pos = f64::floor(statistical::mean(&coordinates)); // for other inputs, ceil()

    coordinates.iter().map(|f| (f - pos).abs() as usize).map(|f| ((f * (f + 1)) / 2)).sum()
}