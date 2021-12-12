use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day12.txt");

fn build_graph<'a>(pairs: &[(&'a str, &'a str)]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

    for (a, b) in pairs {
        graph.entry(a).or_insert(vec![]).push(b);
        graph.entry(b).or_insert(vec![]).push(a);
    }

    graph
}

fn dfs<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, start: &str, marked: &mut Vec<&'a str>, can_twice: bool) -> usize {
    if start == "end" {
        return 1;
    }

    let mut a: usize = 0;

    for x in &graph[start] {
        if x.chars().all(|c| c.is_lowercase()) {
            if !marked.contains(x) {
                let mut m = marked.clone();
                m.push(x);
                a += dfs(graph, x, &mut m, can_twice);
            } else if can_twice && (*x != "start" && *x != "end")  {
                let mut m = marked.clone();
                m.push(x);
                a += dfs(graph, x, &mut m, false);
            }
        } else {
            a += dfs(graph, x, marked, can_twice);
        }
    }

    a
}

fn calculate_paths(can_twice: bool) -> usize {
    let pairs = INPUT.lines().flat_map(|l| l.split_once('-')).collect::<Vec<_>>();
    let graph = build_graph(&pairs);

    let mut marked: Vec<&str> = vec!["start"];
    dfs(&graph, "start", &mut marked, can_twice)
}

pub fn part1() -> usize {
    calculate_paths(false)
}

pub fn part2() -> usize {
    calculate_paths(true)
}